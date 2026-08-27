import {
  DatasourceError,
  MiddlewareHost,
  type IConnection,
  type ITransaction,
} from "@datasources/core";
import { requireDriver, type ISqliteDriver } from "./driver.ts";
import { SqliteTransaction } from "./transaction.ts";

export interface SqliteConnectionOptions {
  filename?: string;
  driver?: ISqliteDriver;
}

export class SqliteConnection extends MiddlewareHost implements IConnection {
  filename: string;
  driver: ISqliteDriver;
  opened = false;

  constructor(options: SqliteConnectionOptions = {}) {
    super();
    this.filename = options.filename ?? ":memory:";
    this.driver = requireDriver(options.driver);
  }

  async open(): Promise<void> {
    return this.run("open", async () => {
      if (this.opened) {
        return;
      }
      await this.driver.open(this.filename);
      this.opened = true;
    });
  }

  async close(): Promise<void> {
    return this.run("close", async () => {
      await this.driver.close();
      this.opened = false;
    });
  }

  async startTransaction(): Promise<ITransaction> {
    return this.run("startTransaction", async () => {
      this.ensureOpen();
      await this.driver.begin();
      return new SqliteTransaction(this);
    });
  }

  ensureOpen(): void {
    if (!this.opened) throw new DatasourceError("Connection is not open");
  }
}
