import {
  type IDatasourceFactory,
  type DatasourceOptions,
  type ICommand,
  type IConnection,
  type IDatasource,
  type IReadonlyRepository,
  type ICRUDRepository,
  StubRepository,
} from "@datasources/core";
import { YamlConnection } from "./connection.ts";
import { YamlCommand } from "./command.ts";

export class YamlDatasource implements IDatasource, IDatasourceFactory {
  static readonly instance = new YamlDatasource();
  readonly provider = "yaml";
  private connection?: IConnection;
  create = (_options?: DatasourceOptions): IDatasource => new YamlDatasource();
  private defaultConnection = (): IConnection =>
    (this.connection ??= new YamlConnection());
  createConnection = (): Promise<IConnection> =>
    Promise.resolve(this.defaultConnection());
  createCommand = (connection?: IConnection): ICommand =>
    new YamlCommand(connection ?? this.defaultConnection());
  createRepository = <T>(connection?: IConnection): ICRUDRepository<T> =>
    new StubRepository<T>(this.provider, connection ?? this.defaultConnection());
  createReadonlyRepository = <T>(
    connection?: IConnection,
  ): IReadonlyRepository<T> => this.createRepository(connection);
}
