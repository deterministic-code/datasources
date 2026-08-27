import { StubConnection } from "@datasources/core";

export class SqlServerConnection extends StubConnection {
  constructor() {
    super("sqlserver");
  }
}
