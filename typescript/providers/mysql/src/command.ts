import {
  ArrayReader,
  CommandMiddlewareHost,
  DatasourceError,
  ExecuteStoredProcedureStatement,
  Native,
  ParameterDirection,
  ReturnStatement,
  SelectStatement,
  Response,
  splitStatements,
  statementReader,
  type Cells,
  type ICommand,
  type IReader,
  type IStatement,
  type Parameter,
} from "@datasources/core";
import { MysqlStatementFactory } from "./MysqlStatementFactory.ts";
import { bindParameters, toMysql } from "./bind.ts";
import type { MysqlConnection } from "./connection.ts";
import type { MysqlExecuteResult } from "./i-mysql-engine.ts";
import type { Scalar } from "./scalar.ts";

const outputs = (parameters?: Parameter[]) =>
  parameters?.filter(
    (parameter) =>
      parameter.direction && parameter.direction !== ParameterDirection.Input,
  ) ?? [];

export class MysqlCommand extends CommandMiddlewareHost implements ICommand {
  constructor(readonly connection: MysqlConnection) {
    super();
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
      return {
        value: statementReader(statements, (sql) => this.query(sql, bound)),
        rows: [],
      };
    });
  }

  async executeReaderStatement(
    statement: IStatement,
    parameters?: Parameter[],
  ): Promise<IReader<Cells>> {
    return this.executeReaderNative(
      new Native(new MysqlStatementFactory().transform(statement).getText()),
      parameters,
    );
  }

  async executeStatement(statement: IStatement, parameters?: Parameter[]): Promise<Response> {
    if (statement instanceof SelectStatement || statement instanceof ReturnStatement)
      return this.firstCell(statement, parameters);
    if (statement instanceof ExecuteStoredProcedureStatement && statement.name && outputs(parameters).length)
      return this.executeProcedure(statement.getText(), parameters);
    return this.executeNative(
      new Native(new MysqlStatementFactory().transform(statement).getText()),
      parameters,
    );
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
}
