import {
  type DatasourceFactory,
  type ICommand,
  type IConnection,
  type IDatasource,
  type IReadonlyRepository,
  type IRepository,
  StubRepository,
} from "@datasources/core";
import { TextConnection } from "./connection.ts";
import { TextCommand } from "./command.ts";

export class TextDatasource implements IDatasource, DatasourceFactory {
  static readonly factory = new TextDatasource();
  readonly provider = "text";
  create = (_options?: unknown): IDatasource => new TextDatasource();
  private newConnection = (): IConnection => new TextConnection();
  createConnection = (): Promise<IConnection> =>
    Promise.resolve(this.newConnection());
  createCommand = (connection?: IConnection): ICommand =>
    new TextCommand(connection ?? this.newConnection());
  createRepository = <T = unknown>(connection?: IConnection): IRepository<T> =>
    new StubRepository<T>(this.provider, connection ?? this.newConnection());

  createReadonlyRepository = <T = unknown>(
    connection?: IConnection,
  ): IReadonlyRepository<T> => this.createRepository(connection);
}
