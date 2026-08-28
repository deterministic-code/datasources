import { IsolationLevel, type IsolationLevelName } from "@datasources/core";
import { toMysql } from "./bind.ts";
import type {
  IMysqlClient,
  IMysqlDriver,
  MysqlCreateConnection,
  MysqlExecuteResult,
} from "./i-mysql-driver.ts";
import type { Cells, Scalar } from "./scalar.ts";

const isoSql = (isolation: IsolationLevelName): string =>
  isolation === IsolationLevel.ReadUncommitted
    ? "READ UNCOMMITTED"
    : isolation === IsolationLevel.ReadCommitted
      ? "READ COMMITTED"
      : "REPEATABLE READ";

export class Mysql2Driver implements IMysqlDriver {
  private client: IMysqlClient | undefined;

  constructor(
    private readonly url: string,
    private readonly createConnection: MysqlCreateConnection,
  ) {}

  async open(_url: string): Promise<void> {
    if (this.client) return;
    this.client = await this.createConnection(this.url);
    await this.client.query(
      "SET SESSION sql_mode = CONCAT(@@sql_mode, ',ANSI_QUOTES')",
    );
  }

  async close(): Promise<void> {
    await this.client?.end();
    this.client = undefined;
  }

  async execute(
    sql: string,
    params?: Record<string, Scalar>,
  ): Promise<MysqlExecuteResult> {
    if (!sql.trim()) return { rowsAffected: 0 };
    const [result] = await this.require().query(toMysql(sql), params);
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
    const [rows] = await this.require().query(toMysql(sql), params);
    return Array.isArray(rows) ? rows : [];
  }

  async begin(
    isolation: IsolationLevelName = IsolationLevel.Serializable,
  ): Promise<void> {
    await this.require().query(
      `SET TRANSACTION ISOLATION LEVEL ${isoSql(isolation)}`,
    );
    await this.require().beginTransaction();
  }

  async commit(): Promise<void> {
    await this.require().commit();
  }

  async rollback(): Promise<void> {
    await this.require().rollback();
  }

  private require(): IMysqlClient {
    if (!this.client) throw new Error("MySQL driver is not open");
    return this.client;
  }
}
