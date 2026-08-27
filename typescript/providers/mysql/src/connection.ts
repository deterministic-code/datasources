import { StubConnection } from "@datasources/core";

export class MysqlConnection extends StubConnection {
  constructor() {
    super("mysql");
  }
}
