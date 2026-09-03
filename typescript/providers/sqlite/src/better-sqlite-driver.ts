import type { IsolationLevelName } from "@datasources/core";
import type {
  ISqliteDriver,
  SqliteDatabaseCtor,
  SqliteExecuteResult,
} from "./i-sqlite-driver.ts";
import { SqliteCommandDriver } from "./sqlite-command-driver.ts";
import { SqliteConnectionDriver } from "./sqlite-connection-driver.ts";
import { SqliteTransactionDriver } from "./sqlite-transaction-driver.ts";

export class BetterSqliteDriver implements ISqliteDriver {
  private readonly connection: SqliteConnectionDriver;
  private readonly commands: SqliteCommandDriver;
  private readonly transactions: SqliteTransactionDriver;

  constructor(filename: string, Database: SqliteDatabaseCtor) {
    this.connection = new SqliteConnectionDriver(filename, Database);
    this.commands = new SqliteCommandDriver(this.connection);
    this.transactions = new SqliteTransactionDriver(this.commands);
  }

  open = (filename: string): Promise<void> => this.connection.open(filename);
  close = (): Promise<void> => this.connection.close();
  execute = (
    sql: string,
    params?: Record<string, unknown>,
  ): Promise<SqliteExecuteResult> => this.commands.execute(sql, params);
  query = (
    sql: string,
    params?: Record<string, unknown>,
  ): Promise<Record<string, unknown>[]> => this.commands.query(sql, params);
  begin = (isolation?: IsolationLevelName): Promise<void> =>
    this.transactions.begin(isolation);
  commit = (): Promise<void> => this.transactions.commit();
  rollback = (): Promise<void> => this.transactions.rollback();
}
