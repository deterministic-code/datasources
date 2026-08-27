import { StubConnection } from "@datasources/core";

export class XmlConnection extends StubConnection {
  constructor() {
    super("xml");
  }
}
