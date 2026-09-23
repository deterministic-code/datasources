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
import { MysqlCommand } from "./command.ts";
import type { MysqlConnectionOptions } from "./connection-options.ts";
import { mysql2Connect, requireDriver, type IMysqlDriver } from "./driver.ts";
import type { Mysql2Module } from "./i-mysql-driver.ts";
import { Mysql2Driver } from "./mysql2-driver.ts";
import { MysqlTransaction } from "./transaction.ts";

const require = createRequire(import.meta.url);

export class MysqlConnection implements IConnection {
  readonly provider = "mysql";
  url: string;
  driver: IMysqlDriver;
  isOpen = false;

  static from = (options?: DatasourceOptions): MysqlConnection => {
    const url = options?.url ?? options?.database_url ?? "";
    const mysql = require("mysql2/promise") as Mysql2Module;
    return new MysqlConnection({
      url,
      driver: new Mysql2Driver(url, mysql2Connect(mysql)),
    });
  };



  constructor(options: MysqlConnectionOptions = {}) {
    this.url = options.url ?? "";
    this.driver = requireDriver(options.driver);
  }

  createCommand = (): ICommand => new MysqlCommand(this);

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
    return new MysqlTransaction(this, isolation);
  }

  ensureOpen(): void {
    if (!this.isOpen) throw new DatasourceError("Connection is not open");
  }
}
