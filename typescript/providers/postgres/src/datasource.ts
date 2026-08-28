import type {
  DatasourceFactory,
  ICommand,
  IConnection,
  IDatasource,
  IReadonlyRepository,
  IRepository,
} from "@datasources/core";
import {
  PostgresConnection,
  type PostgresConnectionOptions,
} from "./connection.ts";
import { PostgresCommand } from "./command.ts";
import { PostgresRepository } from "./repository.ts";
import { requireDriver } from "./driver.ts";

export interface PostgresDatasourceOptions extends PostgresConnectionOptions {
  table?: string;
}

export class PostgresDatasource implements IDatasource, DatasourceFactory {
  static readonly factory = new PostgresDatasource();
  static readonly provider = "postgres";
  readonly provider = "postgres";

  constructor(private readonly options: PostgresDatasourceOptions = {}) {}

  static create = (options?: unknown): IDatasource =>
    new PostgresDatasource((options ?? {}) as PostgresDatasourceOptions);

  create = (options?: unknown): IDatasource => PostgresDatasource.create(options);

  private newConnection = (): PostgresConnection => {
    requireDriver(this.options.driver);
    return new PostgresConnection(this.options);
  };

  createConnection = (): Promise<IConnection> =>
    Promise.resolve(this.newConnection());

  createCommand = (connection?: IConnection): ICommand =>
    new PostgresCommand(
      (connection as PostgresConnection | undefined) ?? this.newConnection(),
    );

  createRepository = <T>(connection?: IConnection): IRepository<T> =>
    new PostgresRepository<T>(
      (connection as PostgresConnection | undefined) ?? this.newConnection(),
      { table: this.options.table },
    );

  createReadonlyRepository = <T>(
    connection?: IConnection,
  ): IReadonlyRepository<T> => this.createRepository(connection);
}
