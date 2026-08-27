import { StubConnection } from "@datasources/core";

export class DynamoDbConnection extends StubConnection {
  constructor() {
    super("dynamodb");
  }
}
