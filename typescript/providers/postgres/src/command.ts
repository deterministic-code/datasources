import { StubCommand, type IConnection } from "@datasources/core";

export class PostgresCommand extends StubCommand {
  constructor(connection: IConnection) {
    super("postgres", connection);
  }
}
