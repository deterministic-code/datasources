import {
  StubConnection,
  StubRepository,
  type DatasourceOptions,
  type ICommand,
  type IConnection,
} from "@datasources/core";
import { OracleCommand } from "./command.ts";

export class OracleConnection extends StubConnection {
  constructor(_options?: DatasourceOptions) {
    super("oracle");
  }

  createCommand = (): ICommand => new OracleCommand(this);

  static from = (options?: DatasourceOptions): OracleConnection =>
    new OracleConnection(options);

  static repository = (connection: IConnection) =>
    new StubRepository(connection.provider, connection);
}
