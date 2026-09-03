import type { SqliteConnectionDriver } from "./sqlite-connection-driver.ts";
import type { SqliteExecuteResult } from "./i-sqlite-driver.ts";

export class SqliteCommandDriver {
  constructor(private readonly connection: SqliteConnectionDriver) {}

  async execute(
    sql: string,
    params?: Record<string, unknown>,
  ): Promise<SqliteExecuteResult> {
    if (!sql.trim()) return { rowsAffected: 0 };
    const info = this.connection.require().prepare(sql).run(params);
    return { rowsAffected: info.changes, result: info.lastInsertRowid };
  }

  async query(
    sql: string,
    params?: Record<string, unknown>,
  ): Promise<Record<string, unknown>[]> {
    if (!sql.trim()) return [];
    return this.connection.require().prepare(sql).all(params);
  }

  exec(sql: string): void {
    this.connection.require().exec(sql);
  }
}
