import {
  type DatasourceFactory,
  type ICommand,
  type IConnection,
  type IDatasource,
  type IRepository,
  StubRepository,
} from "@datasources/core";
import { PostgresConnection } from "./connection.ts";
import { PostgresCommand } from "./command.ts";

export class PostgresDatasource implements IDatasource, DatasourceFactory {
  static readonly factory = new PostgresDatasource();
  readonly provider = "postgres";
  create = (_options?: unknown): IDatasource => new PostgresDatasource();
  private newConnection = (): IConnection => new PostgresConnection();
  createConnection = (): Promise<IConnection> =>
    Promise.resolve(this.newConnection());
  createCommand = (connection?: IConnection): ICommand =>
    new PostgresCommand(connection ?? this.newConnection());
  createRepository = <T = unknown>(connection?: IConnection): IRepository<T> =>
    new StubRepository<T>(this.provider, connection ?? this.newConnection());
}
