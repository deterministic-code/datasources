import type { IsolationLevelName } from "@datasources/core";
import type {
  IPostgresDriver,
  PgClientCtor,
  PostgresExecuteResult,
} from "./i-postgres-driver.ts";
import { PostgresCommandDriver } from "./postgres-command-driver.ts";
import { PostgresConnectionDriver } from "./postgres-connection-driver.ts";
import { PostgresTransactionDriver } from "./postgres-transaction-driver.ts";
import type { Cells, Scalar } from "./scalar.ts";

export class PgDriver implements IPostgresDriver {
  private readonly connection: PostgresConnectionDriver;
  private readonly commands: PostgresCommandDriver;
  private readonly transactions: PostgresTransactionDriver;

  constructor(url: string, Client: PgClientCtor, schema?: string) {
    this.connection = new PostgresConnectionDriver(url, Client, schema);
    this.commands = new PostgresCommandDriver(this.connection);
    this.transactions = new PostgresTransactionDriver(this.commands);
  }

  open = (url: string): Promise<void> => this.connection.open(url);
  close = (): Promise<void> => this.connection.close();
  execute = (
    sql: string,
    params?: Record<string, Scalar>,
  ): Promise<PostgresExecuteResult> => this.commands.execute(sql, params);
  query = (sql: string, params?: Record<string, Scalar>): Promise<Cells[]> =>
    this.commands.query(sql, params);
  begin = (isolation?: IsolationLevelName): Promise<void> =>
    this.transactions.begin(isolation);
  commit = (): Promise<void> => this.transactions.commit();
  rollback = (): Promise<void> => this.transactions.rollback();
}
