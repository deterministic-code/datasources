import {
  type DatasourceFactory,
  type ICommand,
  type IConnection,
  type IDatasource,
  type IRepository,
  StubRepository,
} from "@datasources/core";
import { YamlConnection } from "./connection.ts";
import { YamlCommand } from "./command.ts";

export class YamlDatasource implements IDatasource, DatasourceFactory {
  static readonly factory = new YamlDatasource();
  readonly provider = "yaml";
  create = (_options?: unknown): IDatasource => new YamlDatasource();
  private newConnection = (): IConnection => new YamlConnection();
  createConnection = (): Promise<IConnection> =>
    Promise.resolve(this.newConnection());
  createCommand = (connection?: IConnection): ICommand =>
    new YamlCommand(connection ?? this.newConnection());
  createRepository = <T = unknown>(connection?: IConnection): IRepository<T> =>
    new StubRepository<T>(this.provider, connection ?? this.newConnection());
}
