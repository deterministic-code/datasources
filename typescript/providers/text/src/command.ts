import { StubCommand, type IConnection } from "@datasources/core";

export class TextCommand extends StubCommand {
  constructor(connection: IConnection) {
    super("text", connection);
  }
}
