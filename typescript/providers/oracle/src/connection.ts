import { StubConnection } from "@datasources/core";

export class OracleConnection extends StubConnection {
  constructor() {
    super("oracle");
  }
}
