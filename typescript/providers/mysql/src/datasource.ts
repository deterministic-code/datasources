import {
  type DatasourceFactory,
  type ICommand,
  type IConnection,
  type IDatasource,
  type IRepository,
  StubRepository,
} from "@datasources/core";
import { MysqlConnection } from "./connection.ts";
import { MysqlCommand } from "./command.ts";

export class MysqlDatasource implements IDatasource, DatasourceFactory {
  static readonly factory = new MysqlDatasource();
  readonly provider = "mysql";
  create = (_options?: unknown): IDatasource => new MysqlDatasource();
  private newConnection = (): IConnection => new MysqlConnection();
  createConnection = (): Promise<IConnection> =>
    Promise.resolve(this.newConnection());
  createCommand = (connection?: IConnection): ICommand =>
    new MysqlCommand(connection ?? this.newConnection());
  createRepository = <T = unknown>(connection?: IConnection): IRepository<T> =>
    new StubRepository<T>(this.provider, connection ?? this.newConnection());
}
