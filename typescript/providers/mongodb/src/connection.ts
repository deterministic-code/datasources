import { StubConnection } from "@datasources/core";

export class MongoDbConnection extends StubConnection {
  constructor() {
    super("mongodb");
  }
}
