import {
  StubCommand,
  StubConnection,
  type DatasourceOptions,
  type ICommand,
} from "@datasources/core";

export class XmlConnection extends StubConnection {
  constructor(_options?: DatasourceOptions) {
    super("xml");
  }

  createCommand = (): ICommand => new StubCommand(this.provider, this);

  static from = (options?: DatasourceOptions): XmlConnection =>
    new XmlConnection(options);


}
