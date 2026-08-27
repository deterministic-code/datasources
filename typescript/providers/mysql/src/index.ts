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

export class MysqlConnection extends StubConnection {
  constructor() {
    super("mysql");
  }
}

export class MysqlDatasource implements IDatasource, DatasourceFactory {
  static readonly factory = new MysqlDatasource();
  readonly provider = "mysql";
  create = (_options?: unknown): IDatasource => new MysqlDatasource();
  private newConnection = (): IConnection => new MysqlConnection();
  createConnection = (): Promise<IConnection> => Promise.resolve(this.newConnection());
  createCommand = (connection?: IConnection): ICommand =>
    new StubCommand(this.provider, connection ?? this.newConnection());
  createRepository = <T = unknown>(
    connection?: IConnection,
    _options?: RepositoryOptions,
  ): IRepository<T> =>
    new StubRepository<T>(this.provider, connection ?? this.newConnection());
}

export default MysqlDatasource;
