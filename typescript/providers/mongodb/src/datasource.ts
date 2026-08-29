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
import { MongoDbConnection } from "./connection.ts";
import { MongoDbCommand } from "./command.ts";

export class MongoDbDatasource implements IDatasource, DatasourceFactory {
  static readonly factory = new MongoDbDatasource();
  readonly provider = "mongodb";
  private connection?: IConnection;
  create = (_options?: DatasourceOptions): IDatasource => new MongoDbDatasource();
  private defaultConnection = (): IConnection =>
    (this.connection ??= new MongoDbConnection());
  createConnection = (): Promise<IConnection> =>
    Promise.resolve(this.defaultConnection());
  createCommand = (connection?: IConnection): ICommand =>
    new MongoDbCommand(connection ?? this.defaultConnection());
  createRepository = <T>(connection?: IConnection): IRepository<T> =>
    new StubRepository<T>(this.provider, connection ?? this.defaultConnection());
  createReadonlyRepository = <T>(
    connection?: IConnection,
  ): IReadonlyRepository<T> => this.createRepository(connection);
}
