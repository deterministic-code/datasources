import {
  type DatasourceFactory,
  type ICommand,
  type IConnection,
  type IDatasource,
  type IRepository,
  StubRepository,
} from "@datasources/core";
import { SqlServerConnection } from "./connection.ts";
import { SqlServerCommand } from "./command.ts";

export class SqlServerDatasource implements IDatasource, DatasourceFactory {
  static readonly factory = new SqlServerDatasource();
  readonly provider = "sqlserver";
  create = (_options?: unknown): IDatasource => new SqlServerDatasource();
  private newConnection = (): IConnection => new SqlServerConnection();
  createConnection = (): Promise<IConnection> =>
    Promise.resolve(this.newConnection());
  createCommand = (connection?: IConnection): ICommand =>
    new SqlServerCommand(connection ?? this.newConnection());
  createRepository = <T = unknown>(connection?: IConnection): IRepository<T> =>
    new StubRepository<T>(this.provider, connection ?? this.newConnection());
}
