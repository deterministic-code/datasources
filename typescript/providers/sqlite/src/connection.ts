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
import "./converters/SqliteTypes.ts";
import { SqliteCommand } from "./command.ts";
import type { SqliteConnectionOptions } from "./connection-options.ts";
import { requireDriver, type ISqliteDriver } from "./driver.ts";
import { filenameOf } from "./sqlitePath.ts";
import { SqliteTransaction } from "./transaction.ts";

export class SqliteConnection implements IConnection {
  readonly provider = "sqlite";
  filename: string;
  driver: ISqliteDriver;
  isOpen = false;

  static from = (options?: DatasourceOptions): SqliteConnection =>
    new SqliteConnection({
      filename: options?.filename,
      url: options?.url ?? options?.database_url,
    });



  constructor(options: SqliteConnectionOptions = {}) {
    this.filename = filenameOf(options);
    this.driver = requireDriver(options.driver);
  }

  createCommand = (): ICommand => new SqliteCommand(this);

  async open(): Promise<void> {
    if (this.isOpen) {
      return;
    }
    await this.driver.open(this.filename);
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
    return new SqliteTransaction(this, isolation);
  }

  ensureOpen(): void {
    if (!this.isOpen) throw new DatasourceError("Connection is not open");
  }
}
