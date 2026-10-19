import {
  StubCommand,
  StubConnection,
  type DatasourceOptions,
  type ICommand,
} from "@deterministic-code/datasources";

export class MongoDbConnection extends StubConnection {
  constructor(_options?: DatasourceOptions) {
    super("mongodb");
  }

  createCommand = (): ICommand => new StubCommand(this.provider, this);

  static from = (options?: DatasourceOptions): MongoDbConnection =>
    new MongoDbConnection(options);


}
