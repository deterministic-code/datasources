import { StubConnection } from "@datasources/core";

export class YamlConnection extends StubConnection {
  constructor() {
    super("yaml");
  }
}
