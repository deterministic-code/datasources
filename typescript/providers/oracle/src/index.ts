import {
  type DatasourceFactory,
  type ICommand,
  type IConnection,
  type IDatasource,
  type IRepository,
  StubCommand,
  StubConnection,
  StubRepository,
} from "@datasources/core";

export class OracleConnection extends StubConnection {
  constructor() {
    super("oracle");
  }
}

export class OracleCommand extends StubCommand {
  constructor(connection: IConnection) {
    super("oracle", connection);
  }
}

export class OracleDatasource implements IDatasource, DatasourceFactory {
  static readonly factory = new OracleDatasource();
  readonly provider = "oracle";
  create = (_options?: unknown): IDatasource => new OracleDatasource();
  private newConnection = (): IConnection => new OracleConnection();
  createConnection = (): Promise<IConnection> => Promise.resolve(this.newConnection());
  createCommand = (connection?: IConnection): ICommand =>
    new OracleCommand(connection ?? this.newConnection());
  createRepository = <T = unknown>(connection?: IConnection): IRepository<T> =>
    new StubRepository<T>(this.provider, connection ?? this.newConnection());
}

export default OracleDatasource;
