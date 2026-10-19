import {
  StubCommand,
  StubConnection,
  type DatasourceOptions,
  type ICommand,
} from "@deterministic-code/datasources";

export class YamlConnection extends StubConnection {
  constructor(_options?: DatasourceOptions) {
    super("yaml");
  }

  createCommand = (): ICommand => new StubCommand(this.provider, this);

  static from = (options?: DatasourceOptions): YamlConnection =>
    new YamlConnection(options);


}
