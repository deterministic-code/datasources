import {
  ArrayReader,
  CommandMiddlewareHost,
  FirstResultReader,
  ExecuteStoredProcedureStatement,
  Native,
  ParameterDirection,
  StatementTextRenderer,
  Response,
  splitStatements,
  statementParameters,
  StreamingReader,
  type Cells,
  type ICommand,
  type IReader,
  type IStatement,
  type Parameter,
} from "@deterministic-code/datasources";
import { bindParameters } from "./bind.ts";
import { MysqlDriver } from "./MysqlDriver.ts";
import type { MysqlConnection } from "./connection.ts";
import type { MysqlExecuteResult } from "./i-mysql-engine.ts";
import type { Scalar } from "./scalar.ts";
import { MysqlStatementRenderer } from "./MysqlStatementRenderer.ts";

const outputs = (parameters?: Parameter[]) =>
  parameters?.filter(
    (parameter) =>
      parameter.direction && parameter.direction !== ParameterDirection.Input,
  ) ?? [];

export class MysqlCommand extends CommandMiddlewareHost implements ICommand {
  private readonly statements: StatementTextRenderer;

  constructor(readonly connection: MysqlConnection, statements?: StatementTextRenderer) {
    super();
    this.statements = statements ?? new StatementTextRenderer(t => new MysqlStatementRenderer(t));
  }

  async executeReaderNative(
    text: Native,
    parameters?: Parameter[],
  ): Promise<IReader<Cells>> {
    return this.run(text.text, parameters, async (query, params) => {
      if (!query.trim()) return { value: new ArrayReader([]), rows: [] };
      this.connection.ensureOpen();
      const statements = splitStatements(query);
      if (statements.length === 0) return { value: new ArrayReader([]), rows: [] };
      const bound = bindParameters(params);
      const sets: Cells[][] = [];
      for (const sql of statements) {
        const [result] = await this.connection.require().query(MysqlDriver.placeholders(sql), bound);
        if (!Array.isArray(result)) continue;
        if (result.length > 0 && Array.isArray(result[0]))
          sets.push(...(result as Cells[][]));
        else sets.push(result as Cells[]);
      }
      return FirstResultReader.capture(resultSetsReader(sets));
    });
  }

  async executeReaderStatement(
    statement: IStatement,
    parameters?: Parameter[],
  ): Promise<IReader<Cells>> {
    return this.executeReaderNative(new Native(this.statements.render(statement)), parameters);
  }

  async executeStatement(statement: IStatement, parameters?: Parameter[]): Promise<Response> {
    if (
      statement.constructor.name === "ExecuteStoredProcedureStatement" &&
      (statement as ExecuteStoredProcedureStatement).name &&
      outputs(parameters).length
    )
      return this.executeProcedure(this.statements.render(statement), parameters);
    return this.readsCell(statement)
      ? this.firstCell(statement, parameters)
      : this.executeNative(new Native(this.statements.render(statement)), parameters);
  }

  async executeNative(text: Native, parameters?: Parameter[]): Promise<Response> {
    return this.run(text.text, parameters, async (query, params) => {
      if (!query.trim()) return { value: new Response(null, 0, params), rows: [] };
      this.connection.ensureOpen();
      const named = bindParameters(params);
      let rowsAffected = 0;
      let result: Scalar = null;
      let parameters = params;
      for (const part of splitStatements(query)) {
        const names = new Set(statementParameters(part));
        const used = Object.fromEntries(
          Object.entries(named).filter(([name]) => names.has(name.replace(/^[$:@]/, ""))),
        );
        const executed = await this.runSql(part, used, params);
        rowsAffected += executed.rowsAffected;
        if (executed.result !== undefined && executed.result !== null) result = executed.result;
        parameters = executed.parameters ?? params;
      }
      return {
        value: new Response(result, rowsAffected, parameters),
        rows: [],
      };
    });
  }

  private async executeProcedure(sql: string, parameters?: Parameter[]): Promise<Response> {
    return this.run(sql, parameters, async (query, params) => {
      this.connection.ensureOpen();
      const outs = outputs(params);
      const client = this.connection.require();
      for (const parameter of params!) {
        if (
          !parameter.direction ||
          parameter.direction === ParameterDirection.Input ||
          parameter.direction === ParameterDirection.InputOutput
        )
          await client.query(`SET @${parameter.name} = :${parameter.name}`, {
            [parameter.name]: parameter.value ?? null,
          });
      }
      await client.query(MysqlDriver.placeholders(query).replace(/:([A-Za-z_][A-Za-z0-9_]*)/g, "@$1"));
      const [rows] = await client.query(
        `SELECT ${outs.map((parameter) => `@${parameter.name} AS \`${parameter.name}\``).join(", ")}`,
      );
      const row = Array.isArray(rows) ? (rows[0] as Cells | undefined) : undefined;
      return {
        value: new Response(
          null,
          0,
          params?.map((parameter) =>
            row && parameter.direction && parameter.direction !== ParameterDirection.Input
              ? { ...parameter, value: cellValue(row[parameter.name]) }
              : parameter,
          ),
        ),
        rows: [],
      };
    });
  }

  private async runSql(
    sql: string,
    params?: Record<string, Scalar>,
    parameters?: Parameter[],
  ): Promise<MysqlExecuteResult> {
    const [result] = await this.connection.require().query(MysqlDriver.placeholders(sql), params);
    if (Array.isArray(result)) {
      const row = result[0] as Cells | undefined;
      const values = row ? Object.values(row) : [];
      return {
        rowsAffected: result.length,
        result: values.length === 0 ? null : (values[0] as Scalar),
        parameters,
      };
    }
    return {
      rowsAffected: result.affectedRows ?? 0,
      result: null,
      parameters,
    };
  }
}

const cellValue = (cell: Scalar | object | undefined): Scalar => {
  if (cell === null || cell === undefined) return null;
  if (typeof cell === "string" || typeof cell === "number" || typeof cell === "boolean" || typeof cell === "bigint")
    return cell;
  if (cell instanceof Uint8Array) return new TextDecoder().decode(cell);
  return JSON.stringify(cell);
};

const resultSetsReader = (sets: Cells[][]): StreamingReader => {
  let set = 0;
  let row = 0;
  return new StreamingReader(
    async () => {
      const current = sets[set]?.[row];
      if (current === undefined) return undefined;
      row += 1;
      return current;
    },
    async () => {
      set += 1;
      row = 0;
      return set < sets.length;
    },
  );
};
