import {
  StubCommand,
  StubConnection,
  type DatasourceOptions,
  type ICommand,
} from "@datasources/core";

export class SqlServerConnection extends StubConnection {
  constructor(_options?: DatasourceOptions) {
    super("sqlserver");
  }

  createCommand = (): ICommand => new StubCommand(this.provider, this);

  static from = (options?: DatasourceOptions): SqlServerConnection =>
    new SqlServerConnection(options);


}
