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

export class PostgresConnection extends StubConnection {
  constructor() {
    super("postgres");
  }
}

export class PostgresCommand extends StubCommand {
  constructor(connection: IConnection) {
    super("postgres", connection);
  }
}

export class PostgresDatasource implements IDatasource, DatasourceFactory {
  static readonly factory = new PostgresDatasource();
  readonly provider = "postgres";
  create = (_options?: unknown): IDatasource => new PostgresDatasource();
  private newConnection = (): IConnection => new PostgresConnection();
  createConnection = (): Promise<IConnection> => Promise.resolve(this.newConnection());
  createCommand = (connection?: IConnection): ICommand =>
    new PostgresCommand(connection ?? this.newConnection());
  createRepository = <T = unknown>(
    connection?: IConnection,
    _options?: RepositoryOptions,
  ): IRepository<T> =>
    new StubRepository<T>(this.provider, connection ?? this.newConnection());
}

export default PostgresDatasource;
