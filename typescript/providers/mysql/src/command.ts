import {
  ArrayReader,
  CommandMiddlewareHost,
  FirstResultReader,
  DatasourceError,
  ExecuteStoredProcedureStatement,
  Native,
  ParameterDirection,
  ReturnStatement,
  SelectStatement,
  StatementTextRenderer,
  Response,
  splitStatements,
  statementReader,
  StreamingReader,
  type Cells,
  type ICommand,
  type IReader,
  type IStatement,
  type Parameter,
} from "@datasources/core";
import { bindParameters, toMysql } from "./bind.ts";
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
      if (statements.some((sql) => /^\s*call\b/i.test(sql))) {
        const sets: Cells[][] = [];
        for (const sql of statements) sets.push(...(await this.callResults(sql, bound)));
        return FirstResultReader.capture(resultSetsReader(sets));
      }
      return FirstResultReader.capture(statementReader(statements, (sql) => this.query(sql, bound)));
    });
  }

  async executeReaderStatement(
    statement: IStatement,
    parameters?: Parameter[],
  ): Promise<IReader<Cells>> {
    return this.executeReaderNative(new Native(this.statements.render(statement)), parameters);
  }

  async executeStatement(statement: IStatement, parameters?: Parameter[]): Promise<Response> {
    if (statement instanceof SelectStatement || statement instanceof ReturnStatement)
      return this.firstCell(statement, parameters);
    if (statement instanceof ExecuteStoredProcedureStatement && statement.name && outputs(parameters).length)
      return this.executeProcedure(this.statements.render(statement), parameters);
    return this.executeNative(new Native(this.statements.render(statement)), parameters);
  }

  private async firstCell(statement: IStatement, parameters?: Parameter[]): Promise<Response> {
    const reader = await this.executeReaderStatement(statement, parameters);
    try {
      if (!(await reader.next())) return new Response(null, 0, parameters);
      const row = reader.current;
      /* v8 ignore next */
      if (!row) throw new DatasourceError("Reader current must be a row");
      const values = Object.values(row);
      return new Response(values.length === 0 ? null : values[0], 1, parameters);
    } finally {
      await reader.dispose();
    }
  }

  async executeNative(text: Native, parameters?: Parameter[]): Promise<Response> {
    return this.run(text.text, parameters, async (query, params) => {
      if (!query.trim()) return { value: new Response(undefined, 0, params), rows: [] };
      this.connection.ensureOpen();
      const result = await this.runSql(query, bindParameters(params), params);
      return {
        value: new Response(result.result, result.rowsAffected, result.parameters ?? params),
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
      await client.query(toMysql(query).replace(/:([A-Za-z_][A-Za-z0-9_]*)/g, "@$1"));
      const [rows] = await client.query(
        `SELECT ${outs.map((parameter) => `@${parameter.name} AS \`${parameter.name}\``).join(", ")}`,
      );
      const row = Array.isArray(rows) ? (rows[0] as Cells | undefined) : undefined;
      return {
        value: new Response(
          undefined,
          0,
          params?.map((parameter) =>
            row && parameter.direction && parameter.direction !== ParameterDirection.Input
              ? { ...parameter, value: row[parameter.name] as Scalar }
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
    const [result] = await this.connection.require().query(toMysql(sql), params);
    return Array.isArray(result)
      ? { rowsAffected: result.length, parameters }
      : { rowsAffected: result.affectedRows ?? 0, result: null, parameters };
  }

  private query(sql: string, params?: Record<string, Scalar>): AsyncIterable<Cells> {
    return this.connection.require().queryRows(toMysql(sql), params);
  }

  private async callResults(sql: string, params?: Record<string, Scalar>): Promise<Cells[][]> {
    const [result] = await this.connection.require().query(toMysql(sql), params);
    if (!Array.isArray(result)) return [[]];
    return result.filter((item): item is Cells[] => Array.isArray(item));
  }
}

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
