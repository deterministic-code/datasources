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

export class MongoDbConnection extends StubConnection {
  constructor() {
    super("mongodb");
  }
}

export class MongoDbCommand extends StubCommand {
  constructor(connection: IConnection) {
    super("mongodb", connection);
  }
}

export class MongoDbDatasource implements IDatasource, DatasourceFactory {
  static readonly factory = new MongoDbDatasource();
  readonly provider = "mongodb";
  create = (_options?: unknown): IDatasource => new MongoDbDatasource();
  private newConnection = (): IConnection => new MongoDbConnection();
  createConnection = (): Promise<IConnection> => Promise.resolve(this.newConnection());
  createCommand = (connection?: IConnection): ICommand =>
    new MongoDbCommand(connection ?? this.newConnection());
  createRepository = <T = unknown>(
    connection?: IConnection,
    _options?: RepositoryOptions,
  ): IRepository<T> =>
    new StubRepository<T>(this.provider, connection ?? this.newConnection());
}

export default MongoDbDatasource;
