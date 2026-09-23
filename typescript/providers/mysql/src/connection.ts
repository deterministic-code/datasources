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
import "./converters/MysqlRowTypeConverters.ts";
import { MysqlCommand } from "./command.ts";
import type { MysqlConnectionOptions } from "./connection-options.ts";
import { requireDriver, type IMysqlDriver } from "./driver.ts";
import { MysqlTransaction } from "./transaction.ts";

export class MysqlConnection implements IConnection {
  readonly provider = "mysql";
  url: string;
  driver: IMysqlDriver;
  isOpen = false;

  static from = (options?: DatasourceOptions): MysqlConnection =>
    new MysqlConnection({
      url: options?.url ?? options?.database_url,
    });



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
