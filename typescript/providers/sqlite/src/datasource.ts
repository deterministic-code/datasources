import type {
  ICommand,
  IConnection,
  IDatasource,
  IRepository,
} from "@datasources/core";
import {
  SqliteConnection,
  type SqliteConnectionOptions,
} from "./connection.ts";
import { SqliteCommand } from "./command.ts";
import { SqliteRepository } from "./repository.ts";
import { requireDriver } from "./driver.ts";

export interface SqliteDatasourceOptions extends SqliteConnectionOptions {
  table?: string;
}

export class SqliteDatasource implements IDatasource {
  static readonly factory = new SqliteDatasource();
  static readonly provider = "sqlite";
  readonly provider = "sqlite";

  constructor(private readonly options: SqliteDatasourceOptions = {}) {}

  static create(options?: unknown): IDatasource {
    return new SqliteDatasource((options ?? {}) as SqliteDatasourceOptions);
  }

  create(options?: unknown): IDatasource {
    return SqliteDatasource.create(options);
  }

  private newConnection(): SqliteConnection {
    requireDriver(this.options.driver);
    return new SqliteConnection(this.options);
  }

  createConnection(): Promise<IConnection> {
    return Promise.resolve(this.newConnection());
  }

  createCommand(connection?: IConnection): ICommand {
    const conn =
      (connection as SqliteConnection | undefined) ?? this.newConnection();
    return new SqliteCommand(conn);
  }

  createRepository<T = unknown>(connection?: IConnection): IRepository<T> {
    const conn =
      (connection as SqliteConnection | undefined) ?? this.newConnection();
    return new SqliteRepository<T>(conn, { table: this.options.table });
  }
}

export { requireDriver } from "./driver.ts";
