import type {
  IDatasourceFactory,
  DatasourceOptions,
  ICommand,
  IConnection,
  IDatasource,
  IReadonlyRepository,
  ICRUDRepository,
} from "@datasources/core";
import { Repository, RowTypeConverter } from "@datasources/core";
import type { PostgresConnectionOptions } from "./connection-options.ts";
import { PostgresConnection } from "./connection.ts";
import { PostgresCommand } from "./command.ts";
import { requireDriver } from "./driver.ts";
import "./converters/PostgresTypes.ts";

export interface PostgresDatasourceOptions extends PostgresConnectionOptions {
  table?: string;
}

export class PostgresDatasource implements IDatasource, IDatasourceFactory {
  static readonly instance = new PostgresDatasource();
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

  createRepository = <T>(connection?: IConnection): ICRUDRepository<T> =>
    new Repository({
      connection:
        (connection as PostgresConnection | undefined) ?? this.defaultConnection(),
      entity: this.options.table ?? "items",
      mapping: new RowTypeConverter(),
    }) as ICRUDRepository<T>;

  createReadonlyRepository = <T>(
    connection?: IConnection,
  ): IReadonlyRepository<T> => this.createRepository(connection);
}
