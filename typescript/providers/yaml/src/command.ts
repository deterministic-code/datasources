import { StubCommand, type IConnection } from "@datasources/core";

export class YamlCommand extends StubCommand {
  constructor(connection: IConnection) {
    super("yaml", connection);
  }
}
