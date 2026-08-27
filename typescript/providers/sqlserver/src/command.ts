import { StubCommand, type IConnection } from "@datasources/core";

export class SqlServerCommand extends StubCommand {
  constructor(connection: IConnection) {
    super("sqlserver", connection);
  }
}
