import {
  type DatasourceFactory,
  type DatasourceOptions,
  type ICommand,
  type IConnection,
  type IDatasource,
  type IReadonlyRepository,
  type IRepository,
  StubRepository,
} from "@datasources/core";
import { SqlServerConnection } from "./connection.ts";
import { SqlServerCommand } from "./command.ts";

export class SqlServerDatasource implements IDatasource, DatasourceFactory {
  static readonly instance = new SqlServerDatasource();
  readonly provider = "sqlserver";
  private connection?: IConnection;
  create = (_options?: DatasourceOptions): IDatasource => new SqlServerDatasource();
  private defaultConnection = (): IConnection =>
    (this.connection ??= new SqlServerConnection());
  createConnection = (): Promise<IConnection> =>
    Promise.resolve(this.defaultConnection());
  createCommand = (connection?: IConnection): ICommand =>
    new SqlServerCommand(connection ?? this.defaultConnection());
  createRepository = <T>(connection?: IConnection): IRepository<T> =>
    new StubRepository<T>(this.provider, connection ?? this.defaultConnection());
  createReadonlyRepository = <T>(
    connection?: IConnection,
  ): IReadonlyRepository<T> => this.createRepository(connection);
}
