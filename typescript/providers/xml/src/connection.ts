import {
  StubConnection,
  type DatasourceOptions,
  type ICommand,
} from "@datasources/core";
import { XmlCommand } from "./command.ts";

export class XmlConnection extends StubConnection {
  constructor(_options?: DatasourceOptions) {
    super("xml");
  }

  createCommand = (): ICommand => new XmlCommand(this);

  static from = (options?: DatasourceOptions): XmlConnection =>
    new XmlConnection(options);


}
