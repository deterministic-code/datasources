import {
  StubConnection,
  type DatasourceOptions,
  type ICommand,
} from "@datasources/core";
import { SqlServerCommand } from "./command.ts";

export class SqlServerConnection extends StubConnection {
  constructor(_options?: DatasourceOptions) {
    super("sqlserver");
  }

  createCommand = (): ICommand => new SqlServerCommand(this);

  static from = (options?: DatasourceOptions): SqlServerConnection =>
    new SqlServerConnection(options);


}
