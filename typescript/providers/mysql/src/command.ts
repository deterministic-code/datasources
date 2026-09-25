import {
  ArrayReader,
  CommandMiddlewareHost,
  ParameterDirection,
  responseOf,
  splitStatements,
  statementReader,
  type Cells,
  type ICommand,
  type IReader,
  type Parameter,
  type Response,
} from "@datasources/core";
import { bindParameters, toMysql } from "./bind.ts";
import type { MysqlConnection } from "./connection.ts";
import type { MysqlExecuteResult } from "./i-mysql-engine.ts";
import type { Scalar } from "./scalar.ts";

const isSelect = (sql: string) =>
  sql.trimStart().toUpperCase().startsWith("SELECT");
const isCall = (sql: string) =>
  sql.trimStart().toUpperCase().startsWith("CALL");
const outputs = (parameters?: Parameter[]) =>
  parameters?.filter(
    (parameter) =>
      parameter.direction && parameter.direction !== ParameterDirection.Input,
  ) ?? [];

const scalarOf = (row: Cells | undefined): Scalar | undefined => {
  const value = row ? Object.values(row)[0] : undefined;
  return value;
};

const firstRow = async (rows: AsyncIterable<Cells>): Promise<Cells | undefined> => {
  for await (const row of rows) return row;
  return undefined;
};

export class MysqlCommand extends CommandMiddlewareHost implements ICommand {
  constructor(readonly connection: MysqlConnection) {
    super();
  }

  async executeReader(
    text: string,
    parameters?: Parameter[],
  ): Promise<IReader<Cells>> {
    return this.run(text, parameters, async (query, params) => {
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

  async execute(text: string, parameters?: Parameter[]): Promise<Response> {
    return this.run(text, parameters, async (query, params) => {
      if (!query.trim()) return { value: responseOf(undefined, 0, params), rows: [] };
      this.connection.ensureOpen();
      const result = await this.runSql(query, bindParameters(params), params);
      return {
        value: responseOf(result.result, result.rowsAffected, result.parameters ?? params),
        rows: [],
      };
    });
  }

  private async runSql(
    sql: string,
    params?: Record<string, Scalar>,
    parameters?: Parameter[],
  ): Promise<MysqlExecuteResult> {
    if (isSelect(sql)) {
      const row = await firstRow(this.query(sql, params));
      return { rowsAffected: row ? 1 : 0, result: scalarOf(row), parameters };
    }
    const outs = outputs(parameters);
    if (isCall(sql) && outs.length && parameters) {
      const client = this.connection.require();
      for (const parameter of parameters) {
        if (
          !parameter.direction ||
          parameter.direction === ParameterDirection.Input ||
          parameter.direction === ParameterDirection.InputOutput
        )
          await client.query(`SET @${parameter.name} = :${parameter.name}`, {
            [parameter.name]: parameter.value ?? null,
          });
      }
      await client.query(toMysql(sql).replace(/:([A-Za-z_][A-Za-z0-9_]*)/g, "@$1"));
      const [rows] = await client.query(
        `SELECT ${outs.map((parameter) => `@${parameter.name} AS \`${parameter.name}\``).join(", ")}`,
      );
      const row = Array.isArray(rows) ? rows[0] : undefined;
      return {
        rowsAffected: 0,
        parameters: parameters.map((parameter) =>
          row &&
          parameter.direction &&
          parameter.direction !== ParameterDirection.Input
            ? { ...parameter, value: row[parameter.name] }
            : parameter,
        ),
      };
    }
    const [result] = await this.connection.require().query(toMysql(sql), params);
    return Array.isArray(result)
      ? { rowsAffected: result.length, parameters }
      : {
          rowsAffected: result.affectedRows ?? 0,
          result: result.insertId,
          parameters,
        };
  }

  private query(sql: string, params?: Record<string, Scalar>): AsyncIterable<Cells> {
    return this.connection.require().queryRows(toMysql(sql), params);
  }
}
