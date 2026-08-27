import {
  type DatasourceFactory,
  type ICommand,
  type IConnection,
  type IDatasource,
  type IRepository,
  StubRepository,
} from "@datasources/core";
import { MongoDbConnection } from "./connection.ts";
import { MongoDbCommand } from "./command.ts";

export class MongoDbDatasource implements IDatasource, DatasourceFactory {
  static readonly factory = new MongoDbDatasource();
  readonly provider = "mongodb";
  create = (_options?: unknown): IDatasource => new MongoDbDatasource();
  private newConnection = (): IConnection => new MongoDbConnection();
  createConnection = (): Promise<IConnection> =>
    Promise.resolve(this.newConnection());
  createCommand = (connection?: IConnection): ICommand =>
    new MongoDbCommand(connection ?? this.newConnection());
  createRepository = <T = unknown>(connection?: IConnection): IRepository<T> =>
    new StubRepository<T>(this.provider, connection ?? this.newConnection());
}
