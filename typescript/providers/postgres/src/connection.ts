import {
  DatasourceError,
  DatasourceUrlParser,
  IsolationLevel,
  type DatasourceOptions,
  type ICommand,
  type IConnection,
  type IsolationLevelName,
  type ITransaction,
} from "@datasources/core";
import { createRequire } from "node:module";
import { PostgresCommand } from "./command.ts";
import type { PostgresConnectionOptions } from "./connection-options.ts";
import { pgClient, type IPgClient, type PgClientCtor, type PgModule } from "./i-postgres-driver.ts";
import { PostgresTransaction } from "./transaction.ts";

const require = createRequire(import.meta.url);

const isoSql = (isolation: IsolationLevelName): string =>
  isolation === IsolationLevel.ReadUncommitted
    ? "READ UNCOMMITTED"
    : isolation === IsolationLevel.ReadCommitted
      ? "READ COMMITTED"
      : isolation === IsolationLevel.RepeatableRead
        ? "REPEATABLE READ"
        : "SERIALIZABLE";

export class PostgresConnection implements IConnection {
  readonly provider = "postgres";
  url: string;
  schema?: string;
  isOpen = false;
  private client?: IPgClient;
  private readonly Client?: PgClientCtor;

  static from = (options?: DatasourceOptions): PostgresConnection => {
    const url = options?.DATASOURCE_URL ?? "";
    return new PostgresConnection({
      url,
      schema: DatasourceUrlParser.parse(url)?.schema,
      Client: pgClient(require("pg") as PgModule),
    });
  };

  constructor(options: PostgresConnectionOptions = {}) {
    this.url = options.url ?? "";
    this.schema = options.schema;
    this.Client = options.Client;
  }

  createCommand = (): ICommand => new PostgresCommand(this);

  async open(): Promise<void> {
    if (this.isOpen) return;
    if (!this.client) {
      if (!this.Client) throw new DatasourceError("Postgres driver is required");
      this.client = new this.Client({ connectionString: this.url });
    }
    await this.client.connect();
    if (this.schema) await this.client.query(`SET search_path TO ${this.schema}`);
    this.isOpen = true;
  }

  async close(): Promise<void> {
    await this.client?.end();
    this.client = undefined;
    this.isOpen = false;
  }

  async startTransaction(
    isolation: IsolationLevelName = IsolationLevel.Serializable,
  ): Promise<ITransaction> {
    this.ensureOpen();
    await this.exec(`BEGIN ISOLATION LEVEL ${isoSql(isolation)}`);
    return new PostgresTransaction(this, isolation);
  }

  ensureOpen(): void {
    if (!this.isOpen) throw new DatasourceError("Connection is not open");
  }

  require(): IPgClient {
    if (!this.client) throw new DatasourceError("Connection is not open");
    return this.client;
  }

  async exec(sql: string): Promise<void> {
    await this.require().query(sql);
  }
}
