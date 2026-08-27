import { StubCommand, type IConnection } from "@datasources/core";

export class OracleCommand extends StubCommand {
  constructor(connection: IConnection) {
    super("oracle", connection);
  }
}
