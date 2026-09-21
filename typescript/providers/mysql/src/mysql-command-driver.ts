import { ParameterDirection, type Parameter } from "@datasources/core";
import { toMysql } from "./bind.ts";
import type { MysqlConnectionDriver } from "./mysql-connection-driver.ts";
import type { MysqlExecuteResult } from "./i-mysql-driver.ts";
import type { Cells, Scalar } from "./scalar.ts";

const isSelect = (sql: string) =>
  sql.trimStart().toUpperCase().startsWith("SELECT");
const isCall = (sql: string) =>
  sql.trimStart().toUpperCase().startsWith("CALL");
const outputs = (parameters?: Parameter[]) =>
  parameters?.filter(
    (parameter) =>
      parameter.direction && parameter.direction !== ParameterDirection.Input,
  ) ?? [];

export class MysqlCommandDriver {
  constructor(private readonly connection: MysqlConnectionDriver) {}

  async execute(
    sql: string,
    params?: Record<string, Scalar>,
    parameters?: Parameter[],
  ): Promise<MysqlExecuteResult> {
    if (!sql.trim()) return { rowsAffected: 0, parameters };
    if (isSelect(sql)) {
      const rows = await this.query(sql, params);
      const row = rows[0];
      return {
        rowsAffected: row ? 1 : 0,
        result: row ? (Object.values(row)[0] as Scalar) : undefined,
        parameters,
      };
    }
    const outs = outputs(parameters);
    if (isCall(sql) && outs.length) {
      const client = this.connection.require();
      for (const parameter of parameters!) {
        if (
          !parameter.direction ||
          parameter.direction === ParameterDirection.Input ||
          parameter.direction === ParameterDirection.InputOutput
        )
          await client.query(`SET @${parameter.name} = :${parameter.name}`, {
            [parameter.name]: (parameter.value ?? null) as Scalar,
          });
      }
      await client.query(toMysql(sql).replace(/:([A-Za-z_][A-Za-z0-9_]*)/g, "@$1"));
      const [rows] = await client.query(
        `SELECT ${outs.map((parameter) => `@${parameter.name} AS \`${parameter.name}\``).join(", ")}`,
      );
      const row = (rows as Cells[])[0];
      return {
        rowsAffected: 0,
        parameters: parameters?.map((parameter) =>
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

  async query(
    sql: string,
    params?: Record<string, Scalar>,
  ): Promise<Cells[]> {
    if (!sql.trim()) return [];
    const [rows] = await this.connection.require().query(toMysql(sql), params);
    return Array.isArray(rows) ? rows : [];
  }
}
