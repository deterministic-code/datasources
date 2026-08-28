import type {
  ICommand,
  IConnection,
  IDatasource,
  IReadonlyRepository,
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

  static create = (options?: unknown): IDatasource =>
    new SqliteDatasource((options ?? {}) as SqliteDatasourceOptions);

  create = (options?: unknown): IDatasource => SqliteDatasource.create(options);

  private newConnection = (): SqliteConnection => {
    requireDriver(this.options.driver);
    return new SqliteConnection(this.options);
  };

  createConnection = (): Promise<IConnection> =>
    Promise.resolve(this.newConnection());

  createCommand = (connection?: IConnection): ICommand =>
    new SqliteCommand(
      (connection as SqliteConnection | undefined) ?? this.newConnection(),
    );

  createRepository = <T = unknown>(connection?: IConnection): IRepository<T> =>
    new SqliteRepository<T>(
      (connection as SqliteConnection | undefined) ?? this.newConnection(),
      { table: this.options.table },
    );

  createReadonlyRepository = <T = unknown>(
    connection?: IConnection,
  ): IReadonlyRepository<T> => this.createRepository(connection);
}

export { requireDriver } from "./driver.ts";
