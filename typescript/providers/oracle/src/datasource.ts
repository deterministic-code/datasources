import {
  type DatasourceFactory,
  type ICommand,
  type IConnection,
  type IDatasource,
  type IRepository,
  StubRepository,
} from "@datasources/core";
import { OracleConnection } from "./connection.ts";
import { OracleCommand } from "./command.ts";

export class OracleDatasource implements IDatasource, DatasourceFactory {
  static readonly factory = new OracleDatasource();
  readonly provider = "oracle";
  create = (_options?: unknown): IDatasource => new OracleDatasource();
  private newConnection = (): IConnection => new OracleConnection();
  createConnection = (): Promise<IConnection> =>
    Promise.resolve(this.newConnection());
  createCommand = (connection?: IConnection): ICommand =>
    new OracleCommand(connection ?? this.newConnection());
  createRepository = <T = unknown>(connection?: IConnection): IRepository<T> =>
    new StubRepository<T>(this.provider, connection ?? this.newConnection());
}
