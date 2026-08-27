import { StubCommand, type IConnection } from "@datasources/core";

export class MongoDbCommand extends StubCommand {
  constructor(connection: IConnection) {
    super("mongodb", connection);
  }
}
