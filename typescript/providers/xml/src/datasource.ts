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
import { XmlConnection } from "./connection.ts";
import { XmlCommand } from "./command.ts";

export class XmlDatasource implements IDatasource, DatasourceFactory {
  static readonly instance = new XmlDatasource();
  readonly provider = "xml";
  private connection?: IConnection;
  create = (_options?: DatasourceOptions): IDatasource => new XmlDatasource();
  private defaultConnection = (): IConnection =>
    (this.connection ??= new XmlConnection());
  createConnection = (): Promise<IConnection> =>
    Promise.resolve(this.defaultConnection());
  createCommand = (connection?: IConnection): ICommand =>
    new XmlCommand(connection ?? this.defaultConnection());
  createRepository = <T>(connection?: IConnection): IRepository<T> =>
    new StubRepository<T>(this.provider, connection ?? this.defaultConnection());
  createReadonlyRepository = <T>(
    connection?: IConnection,
  ): IReadonlyRepository<T> => this.createRepository(connection);
}
