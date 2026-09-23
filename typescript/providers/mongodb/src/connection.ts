import {
  StubConnection,
  type DatasourceOptions,
  type ICommand,
} from "@datasources/core";
import { MongoDbCommand } from "./command.ts";

export class MongoDbConnection extends StubConnection {
  constructor(_options?: DatasourceOptions) {
    super("mongodb");
  }

  createCommand = (): ICommand => new MongoDbCommand(this);

  static from = (options?: DatasourceOptions): MongoDbConnection =>
    new MongoDbConnection(options);


}
