import {
  StubCommand,
  StubConnection,
  type DatasourceOptions,
  type ICommand,
} from "@deterministic-code/datasources";

export class XmlConnection extends StubConnection {
  constructor(_options?: DatasourceOptions) {
    super("xml");
  }

  createCommand = (): ICommand => new StubCommand(this.provider, this);

  static from = (options?: DatasourceOptions): XmlConnection =>
    new XmlConnection(options);


}
