import { StubConnection } from "@datasources/core";

export class TextConnection extends StubConnection {
  constructor() {
    super("text");
  }
}
