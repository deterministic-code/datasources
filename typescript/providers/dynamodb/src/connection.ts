import {
  StubCommand,
  StubConnection,
  type DatasourceOptions,
  type ICommand,
} from "@deterministic-code/datasources";

export class DynamoDbConnection extends StubConnection {
  constructor(_options?: DatasourceOptions) {
    super("dynamodb");
  }

  createCommand = (): ICommand => new StubCommand(this.provider, this);

  static from = (options?: DatasourceOptions): DynamoDbConnection =>
    new DynamoDbConnection(options);


}
