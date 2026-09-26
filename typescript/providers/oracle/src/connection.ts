import {
  StubCommand,
  StubConnection,
  type DatasourceOptions,
  type ICommand,
} from "@datasources/core";

export class OracleConnection extends StubConnection {
  constructor(_options?: DatasourceOptions) {
    super("oracle");
  }

  createCommand = (): ICommand => new StubCommand(this.provider, this);

  static from = (options?: DatasourceOptions): OracleConnection =>
    new OracleConnection(options);


}
