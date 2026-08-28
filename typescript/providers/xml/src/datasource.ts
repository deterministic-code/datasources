import {
  type DatasourceFactory,
  type ICommand,
  type IConnection,
  type IDatasource,
  type IReadonlyRepository,
  type IRepository,
  StubRepository,
} from "@datasources/core";
import { XmlConnection } from "./connection.ts";
import { XmlCommand } from "./command.ts";

export class XmlDatasource implements IDatasource, DatasourceFactory {
  static readonly factory = new XmlDatasource();
  readonly provider = "xml";
  create = (_options?: unknown): IDatasource => new XmlDatasource();
  private newConnection = (): IConnection => new XmlConnection();
  createConnection = (): Promise<IConnection> =>
    Promise.resolve(this.newConnection());
  createCommand = (connection?: IConnection): ICommand =>
    new XmlCommand(connection ?? this.newConnection());
  createRepository = <T = unknown>(connection?: IConnection): IRepository<T> =>
    new StubRepository<T>(this.provider, connection ?? this.newConnection());

  createReadonlyRepository = <T = unknown>(
    connection?: IConnection,
  ): IReadonlyRepository<T> => this.createRepository(connection);
}
