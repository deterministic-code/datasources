import {
  StubConnection,
  type DatasourceOptions,
  type ICommand,
} from "@datasources/core";
import { OracleCommand } from "./command.ts";

export class OracleConnection extends StubConnection {
  constructor(_options?: DatasourceOptions) {
    super("oracle");
  }

  createCommand = (): ICommand => new OracleCommand(this);

  static from = (options?: DatasourceOptions): OracleConnection =>
    new OracleConnection(options);


}
