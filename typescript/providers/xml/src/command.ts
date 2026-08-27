import { StubCommand, type IConnection } from "@datasources/core";

export class XmlCommand extends StubCommand {
  constructor(connection: IConnection) {
    super("xml", connection);
  }
}
