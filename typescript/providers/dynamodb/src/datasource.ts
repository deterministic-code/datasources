import {
  type DatasourceFactory,
  type ICommand,
  type IConnection,
  type IDatasource,
  type IRepository,
  StubRepository,
} from "@datasources/core";
import { DynamoDbConnection } from "./connection.ts";
import { DynamoDbCommand } from "./command.ts";

export class DynamoDbDatasource implements IDatasource, DatasourceFactory {
  static readonly factory = new DynamoDbDatasource();
  readonly provider = "dynamodb";
  create = (_options?: unknown): IDatasource => new DynamoDbDatasource();
  private newConnection = (): IConnection => new DynamoDbConnection();
  createConnection = (): Promise<IConnection> =>
    Promise.resolve(this.newConnection());
  createCommand = (connection?: IConnection): ICommand =>
    new DynamoDbCommand(connection ?? this.newConnection());
  createRepository = <T = unknown>(connection?: IConnection): IRepository<T> =>
    new StubRepository<T>(this.provider, connection ?? this.newConnection());
}
