import {
  DatasourceError,
  IsolationLevel,
  type IConnection,
  type IsolationLevelName,
  type ITransaction,
} from "@datasources/core";
import { requireDriver, type IPostgresDriver } from "./driver.ts";
import { PostgresTransaction } from "./transaction.ts";

export interface PostgresConnectionOptions {
  url?: string;
  schema?: string;
  driver?: IPostgresDriver;
}

export class PostgresConnection implements IConnection {
  url: string;
  schema?: string;
  driver: IPostgresDriver;
  opened = false;

  constructor(options: PostgresConnectionOptions = {}) {
    this.url = options.url ?? "";
    this.schema = options.schema;
    this.driver = requireDriver(options.driver);
  }

  async open(): Promise<void> {
    if (this.opened) return;
    await this.driver.open(this.url);
    this.opened = true;
  }

  async close(): Promise<void> {
    await this.driver.close();
    this.opened = false;
  }

  async startTransaction(
    isolation: IsolationLevelName = IsolationLevel.Serializable,
  ): Promise<ITransaction> {
    this.ensureOpen();
    await this.driver.begin(isolation);
    return new PostgresTransaction(this, isolation);
  }

  ensureOpen(): void {
    if (!this.opened) throw new DatasourceError("Connection is not open");
  }
}
