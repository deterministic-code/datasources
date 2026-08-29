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
import { OracleConnection } from "./connection.ts";
import { OracleCommand } from "./command.ts";

export class OracleDatasource implements IDatasource, DatasourceFactory {
  static readonly instance = new OracleDatasource();
  readonly provider = "oracle";
  private connection?: IConnection;
  create = (_options?: DatasourceOptions): IDatasource => new OracleDatasource();
  private defaultConnection = (): IConnection =>
    (this.connection ??= new OracleConnection());
  createConnection = (): Promise<IConnection> =>
    Promise.resolve(this.defaultConnection());
  createCommand = (connection?: IConnection): ICommand =>
    new OracleCommand(connection ?? this.defaultConnection());
  createRepository = <T>(connection?: IConnection): IRepository<T> =>
    new StubRepository<T>(this.provider, connection ?? this.defaultConnection());
  createReadonlyRepository = <T>(
    connection?: IConnection,
  ): IReadonlyRepository<T> => this.createRepository(connection);
}
