import {
  DatasourceError,
  IsolationLevel,
  CRUDRepository,
  RowTypeConverter,
  type DatasourceOptions,
  type ICommand,
  type IConnection,
  type IsolationLevelName,
  type ITransaction,
} from "@datasources/core";
import "./converters/PostgresTypes.ts";
import { PostgresCommand } from "./command.ts";
import type { PostgresConnectionOptions } from "./connection-options.ts";
import { requireDriver, type IPostgresDriver } from "./driver.ts";
import { PostgresTransaction } from "./transaction.ts";

export class PostgresConnection implements IConnection {
  readonly provider = "postgres";
  url: string;
  schema?: string;
  driver: IPostgresDriver;
  isOpen = false;

  static from = (options?: DatasourceOptions): PostgresConnection =>
    new PostgresConnection({
      url: options?.url ?? options?.database_url,
      schema: options?.schema ?? options?.database_schema,
    });



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
