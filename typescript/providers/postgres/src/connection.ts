import { StubConnection } from "@datasources/core";

export class PostgresConnection extends StubConnection {
  constructor() {
    super("postgres");
  }
}
