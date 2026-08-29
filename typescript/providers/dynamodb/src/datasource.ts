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
import { DynamoDbConnection } from "./connection.ts";
import { DynamoDbCommand } from "./command.ts";

export class DynamoDbDatasource implements IDatasource, DatasourceFactory {
  static readonly instance = new DynamoDbDatasource();
  readonly provider = "dynamodb";
  private connection?: IConnection;
  create = (_options?: DatasourceOptions): IDatasource => new DynamoDbDatasource();
  private defaultConnection = (): IConnection =>
    (this.connection ??= new DynamoDbConnection());
  createConnection = (): Promise<IConnection> =>
    Promise.resolve(this.defaultConnection());
  createCommand = (connection?: IConnection): ICommand =>
    new DynamoDbCommand(connection ?? this.defaultConnection());
  createRepository = <T>(connection?: IConnection): IRepository<T> =>
    new StubRepository<T>(this.provider, connection ?? this.defaultConnection());
  createReadonlyRepository = <T>(
    connection?: IConnection,
  ): IReadonlyRepository<T> => this.createRepository(connection);
}
