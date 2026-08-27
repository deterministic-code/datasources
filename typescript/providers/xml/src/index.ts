import {
  type DatasourceFactory,
  type ICommand,
  type IConnection,
  type IDatasource,
  type IRepository,
  type RepositoryOptions,
  StubCommand,
  StubConnection,
  StubRepository,
} from "@datasources/core";

export class XmlConnection extends StubConnection {
  constructor() {
    super("xml");
  }
}

export class XmlCommand extends StubCommand {
  constructor(connection: IConnection) {
    super("xml", connection);
  }
}

export class XmlDatasource implements IDatasource, DatasourceFactory {
  static readonly factory = new XmlDatasource();
  readonly provider = "xml";
  create = (_options?: unknown): IDatasource => new XmlDatasource();
  private newConnection = (): IConnection => new XmlConnection();
  createConnection = (): Promise<IConnection> => Promise.resolve(this.newConnection());
  createCommand = (connection?: IConnection): ICommand =>
    new XmlCommand(connection ?? this.newConnection());
  createRepository = <T = unknown>(
    connection?: IConnection,
    _options?: RepositoryOptions,
  ): IRepository<T> =>
    new StubRepository<T>(this.provider, connection ?? this.newConnection());
}

export default XmlDatasource;
