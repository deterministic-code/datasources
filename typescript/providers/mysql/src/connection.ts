import {
  DatasourceError,
  IsolationLevel,
  StatementTextRenderer,
  type DatasourceOptions,
  type ICommand,
  type IConnection,
  type IsolationLevelName,
  type ITransaction,
} from "@deterministic-code/datasources";
import { createRequire } from "node:module";
import { MysqlCommand } from "./command.ts";
import type { MysqlConnectionOptions } from "./connection-options.ts";
import {
  mysql2Connect,
  type IMysqlClient,
  type Mysql2Module,
  type MysqlCreateConnection,
} from "./i-mysql-engine.ts";
import { MysqlStatementRenderer } from "./MysqlStatementRenderer.ts";
import { MysqlTransaction } from "./transaction.ts";

const require = createRequire(import.meta.url);

const isoSql = (isolation: IsolationLevelName): string =>
  isolation === IsolationLevel.ReadUncommitted
    ? "READ UNCOMMITTED"
    : isolation === IsolationLevel.ReadCommitted
      ? "READ COMMITTED"
      : "REPEATABLE READ";

export class MysqlConnection implements IConnection {
  readonly provider = "mysql";
  url: string;
  isOpen = false;
  private client?: IMysqlClient;
  private readonly connect?: MysqlCreateConnection;

  static from = (options?: DatasourceOptions): MysqlConnection => {
    const url = options?.DATASOURCE_URL ?? "";
    return new MysqlConnection({
      url,
      connect: mysql2Connect(require("mysql2/promise") as Mysql2Module),
    });
  };

  constructor(options: MysqlConnectionOptions = {}) {
    this.url = options.url ?? "";
    this.connect = options.connect;
  }

  createCommand = (): ICommand =>
    new MysqlCommand(this, new StatementTextRenderer(t => new MysqlStatementRenderer(t)));

  async open(): Promise<void> {
    if (this.isOpen) throw new DatasourceError("Connection is already open");
    if (!this.client) {
      if (!this.connect) throw new DatasourceError("MySQL engine is required");
      this.client = await this.connect(this.url);
    }
    await this.client.query("SET SESSION sql_mode = CONCAT(@@sql_mode, ',ANSI_QUOTES')");
    await this.client.query("SET time_zone = '+00:00'");
    this.isOpen = true;
  }

  async close(): Promise<void> {
    await this.client?.end();
    this.client = undefined;
    this.isOpen = false;
  }

  [Symbol.asyncDispose] = (): Promise<void> => this.close();

  async startTransaction(
    isolation: IsolationLevelName = IsolationLevel.ReadCommitted,
  ): Promise<ITransaction> {
    this.ensureOpen();
    const client = this.require();
    await client.query(`SET TRANSACTION ISOLATION LEVEL ${isoSql(isolation)}`);
    await client.beginTransaction();
    return new MysqlTransaction(this, isolation);
  }

  ensureOpen(): void {
    if (!this.isOpen) throw new DatasourceError("Connection is not open");
  }

  require(): IMysqlClient {
    if (!this.client) throw new DatasourceError("Connection is not open");
    return this.client;
  }
}
