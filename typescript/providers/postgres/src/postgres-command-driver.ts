import { ParameterDirection, type Parameter } from "@datasources/core";
import { toPostgres, withReturning } from "./bind.ts";
import type { PostgresConnectionDriver } from "./postgres-connection-driver.ts";
import type { PostgresExecuteResult } from "./i-postgres-driver.ts";
import type { Cells, Scalar } from "./scalar.ts";

const isSelect = (sql: string) =>
  sql.trimStart().toUpperCase().startsWith("SELECT");
const isCall = (sql: string) =>
  sql.trimStart().toUpperCase().startsWith("CALL");
const hasOutput = (parameters?: Parameter[]) =>
  parameters?.some(
    (parameter) =>
      parameter.direction && parameter.direction !== ParameterDirection.Input,
  ) ?? false;

export class PostgresCommandDriver {
  constructor(private readonly connection: PostgresConnectionDriver) {}

  async execute(
    sql: string,
    params?: Record<string, Scalar>,
    parameters?: Parameter[],
  ): Promise<PostgresExecuteResult> {
    if (!sql.trim()) return { rowsAffected: 0, parameters };
    const rewritten = withReturning(sql);
    if (isSelect(rewritten) || (isCall(rewritten) && hasOutput(parameters))) {
      const rows = await this.query(rewritten, params);
      const row = rows[0];
      return {
        rowsAffected: row ? 1 : 0,
        result: row ? (Object.values(row)[0] as Scalar) : undefined,
        parameters: row && parameters
          ? parameters.map((parameter) =>
              parameter.direction &&
              parameter.direction !== ParameterDirection.Input &&
              parameter.name in row
                ? { ...parameter, value: row[parameter.name] }
                : parameter,
            )
          : parameters,
      };
    }
    const { text, values } = toPostgres(rewritten, params);
    const result = await this.connection.require().query(text, values);
    return {
      rowsAffected: result.rowCount ?? result.rows.length,
      result: result.rows[0]?.id,
      parameters,
    };
  }

  async query(
    sql: string,
    params?: Record<string, Scalar>,
  ): Promise<Cells[]> {
    if (!sql.trim()) return [];
    const { text, values } = toPostgres(sql, params);
    return (await this.connection.require().query(text, values)).rows;
  }

  async exec(sql: string): Promise<void> {
    await this.connection.require().query(sql);
  }
}
