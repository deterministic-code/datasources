import {
  DatasourceError,
  IsolationLevel,
  type IConnection,
  type IsolationLevelName,
  type ITransaction,
} from "@datasources/core";
import { requireDriver, type IMysqlDriver } from "./driver.ts";
import { MysqlTransaction } from "./transaction.ts";

export interface MysqlConnectionOptions {
  url?: string;
  driver?: IMysqlDriver;
}

export class MysqlConnection implements IConnection {
  url: string;
  driver: IMysqlDriver;
  opened = false;

  constructor(options: MysqlConnectionOptions = {}) {
    this.url = options.url ?? "";
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
    return new MysqlTransaction(this, isolation);
  }

  ensureOpen(): void {
    if (!this.opened) throw new DatasourceError("Connection is not open");
  }
}
