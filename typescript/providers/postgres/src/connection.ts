import {
  DatasourceError,
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
import { requireDriver, type IPostgresDriver } from "./driver.ts";
import type { PgModule } from "./i-postgres-driver.ts";
import { PgDriver } from "./pg-driver.ts";
import { PostgresTransaction } from "./transaction.ts";

const require = createRequire(import.meta.url);

export class PostgresConnection implements IConnection {
  readonly provider = "postgres";
  url: string;
  schema?: string;
  driver: IPostgresDriver;
  isOpen = false;

  static from = (options?: DatasourceOptions): PostgresConnection => {
    const url = options?.url ?? options?.database_url ?? "";
    const schema = options?.schema ?? options?.database_schema;
    const { Client } = require("pg") as PgModule;
    return new PostgresConnection({
      url,
      schema,
      driver: new PgDriver(url, Client, schema),
    });
  };



  constructor(options: PostgresConnectionOptions = {}) {
    this.url = options.url ?? "";
    this.schema = options.schema;
    this.driver = requireDriver(options.driver);
  }

  createCommand = (): ICommand => new PostgresCommand(this);

  async open(): Promise<void> {
    if (this.isOpen) return;
    await this.driver.open(this.url);
    this.isOpen = true;
  }

  async close(): Promise<void> {
    await this.driver.close();
    this.isOpen = false;
  }

  async startTransaction(
    isolation: IsolationLevelName = IsolationLevel.Serializable,
  ): Promise<ITransaction> {
    this.ensureOpen();
    await this.driver.begin(isolation);
    return new PostgresTransaction(this, isolation);
  }

  ensureOpen(): void {
    if (!this.isOpen) throw new DatasourceError("Connection is not open");
  }
}
