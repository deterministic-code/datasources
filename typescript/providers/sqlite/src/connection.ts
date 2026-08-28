import {
  DatasourceError,
  IsolationLevel,
  type IConnection,
  type IsolationLevelName,
  type ITransaction,
} from "@datasources/core";
import { requireDriver, type ISqliteDriver } from "./driver.ts";
import { SqliteTransaction } from "./transaction.ts";

export interface SqliteConnectionOptions {
  filename?: string;
  driver?: ISqliteDriver;
}

export class SqliteConnection implements IConnection {
  filename: string;
  driver: ISqliteDriver;
  opened = false;

  constructor(options: SqliteConnectionOptions = {}) {
    this.filename = options.filename ?? ":memory:";
    this.driver = requireDriver(options.driver);
  }

  async open(): Promise<void> {
    if (this.opened) {
      return;
    }
    await this.driver.open(this.filename);
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
    return new SqliteTransaction(this, isolation);
  }

  ensureOpen(): void {
    if (!this.opened) throw new DatasourceError("Connection is not open");
  }
}
