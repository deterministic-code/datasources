import type { IsolationLevelName } from "@datasources/core";
import type {
  IMysqlDriver,
  MysqlCreateConnection,
  MysqlExecuteResult,
} from "./i-mysql-driver.ts";
import { MysqlCommandDriver } from "./mysql-command-driver.ts";
import { MysqlConnectionDriver } from "./mysql-connection-driver.ts";
import { MysqlTransactionDriver } from "./mysql-transaction-driver.ts";
import type { Cells, Scalar } from "./scalar.ts";

export class Mysql2Driver implements IMysqlDriver {
  private readonly connection: MysqlConnectionDriver;
  private readonly commands: MysqlCommandDriver;
  private readonly transactions: MysqlTransactionDriver;

  constructor(url: string, createConnection: MysqlCreateConnection) {
    this.connection = new MysqlConnectionDriver(url, createConnection);
    this.commands = new MysqlCommandDriver(this.connection);
    this.transactions = new MysqlTransactionDriver(this.connection);
  }

  open = (url: string): Promise<void> => this.connection.open(url);
  close = (): Promise<void> => this.connection.close();
  execute = (
    sql: string,
    params?: Record<string, Scalar>,
  ): Promise<MysqlExecuteResult> => this.commands.execute(sql, params);
  query = (sql: string, params?: Record<string, Scalar>): Promise<Cells[]> =>
    this.commands.query(sql, params);
  begin = (isolation?: IsolationLevelName): Promise<void> =>
    this.transactions.begin(isolation);
  commit = (): Promise<void> => this.transactions.commit();
  rollback = (): Promise<void> => this.transactions.rollback();
}
