import { StubCommand, type IConnection } from "@datasources/core";

export class MysqlCommand extends StubCommand {
  constructor(connection: IConnection) {
    super("mysql", connection);
  }
}
