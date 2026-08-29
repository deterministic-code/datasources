import type {
  DatasourceFactory,
  DatasourceOptions,
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

  private connection?: PostgresConnection;

  static create = (options?: DatasourceOptions): IDatasource =>
    new PostgresDatasource((options ?? {}) as PostgresDatasourceOptions);

  create = (options?: DatasourceOptions): IDatasource =>
    PostgresDatasource.create(options);

  private newConnection = (): PostgresConnection => {
    requireDriver(this.options.driver);
    return new PostgresConnection(this.options);
  };

  private defaultConnection = (): PostgresConnection =>
    (this.connection ??= this.newConnection());

  createConnection = (): Promise<IConnection> =>
    Promise.resolve(this.defaultConnection());

  createCommand = (connection?: IConnection): ICommand =>
    new PostgresCommand(
      (connection as PostgresConnection | undefined) ?? this.defaultConnection(),
    );

  createRepository = <T>(connection?: IConnection): IRepository<T> =>
    new PostgresRepository<T>(
      (connection as PostgresConnection | undefined) ?? this.defaultConnection(),
      { table: this.options.table },
    );

  createReadonlyRepository = <T>(
    connection?: IConnection,
  ): IReadonlyRepository<T> => this.createRepository(connection);
}
