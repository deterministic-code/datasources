import {
  StubCommand,
  StubConnection,
  type DatasourceOptions,
  type ICommand,
} from "@datasources/core";

export class TextConnection extends StubConnection {
  constructor(_options?: DatasourceOptions) {
    super("text");
  }

  createCommand = (): ICommand => new StubCommand(this.provider, this);

  static from = (options?: DatasourceOptions): TextConnection =>
    new TextConnection(options);


}
