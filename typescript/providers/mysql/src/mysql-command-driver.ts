import { toMysql } from "./bind.ts";
import type { MysqlConnectionDriver } from "./mysql-connection-driver.ts";
import type { MysqlExecuteResult } from "./i-mysql-driver.ts";
import type { Cells, Scalar } from "./scalar.ts";

export class MysqlCommandDriver {
  constructor(private readonly connection: MysqlConnectionDriver) {}

  async execute(
    sql: string,
    params?: Record<string, Scalar>,
  ): Promise<MysqlExecuteResult> {
    if (!sql.trim()) return { rowsAffected: 0 };
    const [result] = await this.connection.require().query(toMysql(sql), params);
    return Array.isArray(result)
      ? { rowsAffected: result.length }
      : {
          rowsAffected: result.affectedRows ?? 0,
          result: result.insertId,
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
