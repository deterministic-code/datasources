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

export class YamlConnection extends StubConnection {
  constructor() {
    super("yaml");
  }
}

export class YamlCommand extends StubCommand {
  constructor(connection: IConnection) {
    super("yaml", connection);
  }
}

export class YamlDatasource implements IDatasource, DatasourceFactory {
  static readonly factory = new YamlDatasource();
  readonly provider = "yaml";
  create = (_options?: unknown): IDatasource => new YamlDatasource();
  private newConnection = (): IConnection => new YamlConnection();
  createConnection = (): Promise<IConnection> => Promise.resolve(this.newConnection());
  createCommand = (connection?: IConnection): ICommand =>
    new YamlCommand(connection ?? this.newConnection());
  createRepository = <T = unknown>(
    connection?: IConnection,
    _options?: RepositoryOptions,
  ): IRepository<T> =>
    new StubRepository<T>(this.provider, connection ?? this.newConnection());
}

export default YamlDatasource;
