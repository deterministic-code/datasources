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

export class TextConnection extends StubConnection {
  constructor() {
    super("text");
  }
}

export class TextCommand extends StubCommand {
  constructor(connection: IConnection) {
    super("text", connection);
  }
}

export class TextDatasource implements IDatasource, DatasourceFactory {
  static readonly factory = new TextDatasource();
  readonly provider = "text";
  create = (_options?: unknown): IDatasource => new TextDatasource();
  private newConnection = (): IConnection => new TextConnection();
  createConnection = (): Promise<IConnection> => Promise.resolve(this.newConnection());
  createCommand = (connection?: IConnection): ICommand =>
    new TextCommand(connection ?? this.newConnection());
  createRepository = <T = unknown>(
    connection?: IConnection,
    _options?: RepositoryOptions,
  ): IRepository<T> =>
    new StubRepository<T>(this.provider, connection ?? this.newConnection());
}

export default TextDatasource;
