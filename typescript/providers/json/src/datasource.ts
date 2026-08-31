import {
  type DatasourceOptions,
  type ICommand,
  type IConnection,
  type IDatasource,
  type IReadonlyRepository,
  type IRepository,
} from "@datasources/core";
import type { JsonConnectionOptions } from "./connection-options.ts";
import { JsonConnection } from "./connection.ts";
import { JsonCommand } from "./command.ts";
import { JsonRepository } from "./repository.ts";

export class JsonDatasource implements IDatasource {
  static readonly instance = new JsonDatasource();
  static readonly provider = "json";
  readonly provider = "json";

  constructor(private readonly options: JsonConnectionOptions = {}) {}

  private connection?: JsonConnection;

  static create = (options?: DatasourceOptions): IDatasource =>
    new JsonDatasource((options ?? {}) as JsonConnectionOptions);

  create = (options?: DatasourceOptions): IDatasource =>
    JsonDatasource.create(options);

  private newConnection = (): JsonConnection =>
    new JsonConnection(this.options);

  private defaultConnection = (): JsonConnection =>
    (this.connection ??= this.newConnection());

  createConnection = (): Promise<IConnection> =>
    Promise.resolve(this.defaultConnection());

  createCommand = (connection?: IConnection): ICommand =>
    new JsonCommand(
      (connection as JsonConnection | undefined) ?? this.defaultConnection(),
    );

  createRepository = <T>(connection?: IConnection): IRepository<T> =>
    new JsonRepository<T>(
      (connection as JsonConnection | undefined) ?? this.defaultConnection(),
    );

  createReadonlyRepository = <T>(
    connection?: IConnection,
  ): IReadonlyRepository<T> => this.createRepository(connection);
}
