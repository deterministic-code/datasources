import {
  StubConnection,
  type DatasourceOptions,
  type ICommand,
} from "@datasources/core";
import { DynamoDbCommand } from "./command.ts";

export class DynamoDbConnection extends StubConnection {
  constructor(_options?: DatasourceOptions) {
    super("dynamodb");
  }

  createCommand = (): ICommand => new DynamoDbCommand(this);

  static from = (options?: DatasourceOptions): DynamoDbConnection =>
    new DynamoDbConnection(options);


}
