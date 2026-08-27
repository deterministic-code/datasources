import { StubCommand, type IConnection } from "@datasources/core";

export class DynamoDbCommand extends StubCommand {
  constructor(connection: IConnection) {
    super("dynamodb", connection);
  }
}
