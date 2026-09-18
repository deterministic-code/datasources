import {
  type IDatasourceFactory,
  type DatasourceOptions,
  type ICommand,
  type IConnection,
  type IDatasource,
  type IReadonlyRepository,
  type ICRUDRepository,
  StubRepository,
} from "@datasources/core";
import { TextConnection } from "./connection.ts";
import { TextCommand } from "./command.ts";

export class TextDatasource implements IDatasource, IDatasourceFactory {
  static readonly instance = new TextDatasource();
  readonly provider = "text";
  private connection?: IConnection;
  create = (_options?: DatasourceOptions): IDatasource => new TextDatasource();
  private defaultConnection = (): IConnection =>
    (this.connection ??= new TextConnection());
  createConnection = (): Promise<IConnection> =>
    Promise.resolve(this.defaultConnection());
  createCommand = (connection?: IConnection): ICommand =>
    new TextCommand(connection ?? this.defaultConnection());
  createRepository = <T>(connection?: IConnection): ICRUDRepository<T> =>
    new StubRepository<T>(this.provider, connection ?? this.defaultConnection());
  createReadonlyRepository = <T>(
    connection?: IConnection,
  ): IReadonlyRepository<T> => this.createRepository(connection);
}
