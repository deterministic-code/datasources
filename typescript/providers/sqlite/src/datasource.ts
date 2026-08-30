import type {
  DatasourceOptions,
  ICommand,
  IConnection,
  IDatasource,
  IReadonlyRepository,
  IRepository,
} from "@datasources/core";
import { Repository, TypeConverter, type Cells } from "@datasources/core";
import {
  SqliteConnection,
  type SqliteConnectionOptions,
} from "./connection.ts";
import { SqliteCommand } from "./command.ts";
import { requireDriver } from "./driver.ts";

export interface SqliteDatasourceOptions extends SqliteConnectionOptions {
  table?: string;
}

export class SqliteDatasource implements IDatasource {
  static readonly instance = new SqliteDatasource();
  static readonly provider = "sqlite";
  readonly provider = "sqlite";

  constructor(private readonly options: SqliteDatasourceOptions = {}) {}

  private connection?: SqliteConnection;

  static create = (options?: DatasourceOptions): IDatasource =>
    new SqliteDatasource((options ?? {}) as SqliteDatasourceOptions);

  create = (options?: DatasourceOptions): IDatasource =>
    SqliteDatasource.create(options);

  private newConnection = (): SqliteConnection => {
    requireDriver(this.options.driver);
    return new SqliteConnection(this.options);
  };

  private defaultConnection = (): SqliteConnection =>
    (this.connection ??= this.newConnection());

  createConnection = (): Promise<IConnection> =>
    Promise.resolve(this.defaultConnection());

  createCommand = (connection?: IConnection): ICommand =>
    new SqliteCommand(
      (connection as SqliteConnection | undefined) ?? this.defaultConnection(),
    );

  createRepository = <T>(connection?: IConnection): IRepository<T> =>
    new Repository({
      connection: (connection as SqliteConnection | undefined) ?? this.defaultConnection(),
      mapping: new TypeConverter(this.options.table ?? "items"),
    }) as IRepository<T>;

  createReadonlyRepository = <T>(
    connection?: IConnection,
  ): IReadonlyRepository<T> => this.createRepository(connection);
}

export { requireDriver } from "./driver.ts";
