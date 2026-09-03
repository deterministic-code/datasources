import { toPostgres, withReturning } from "./bind.ts";
import type { PostgresConnectionDriver } from "./postgres-connection-driver.ts";
import type { PostgresExecuteResult } from "./i-postgres-driver.ts";
import type { Cells, Scalar } from "./scalar.ts";

export class PostgresCommandDriver {
  constructor(private readonly connection: PostgresConnectionDriver) {}

  async execute(
    sql: string,
    params?: Record<string, Scalar>,
  ): Promise<PostgresExecuteResult> {
    if (!sql.trim()) return { rowsAffected: 0 };
    const rewritten = withReturning(sql);
    const { text, values } = toPostgres(rewritten, params);
    const result = await this.connection.require().query(text, values);
    return {
      rowsAffected: result.rowCount ?? result.rows.length,
      result: result.rows[0]?.id,
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
