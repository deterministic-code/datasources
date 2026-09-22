import {
  StubConnection,
  type DatasourceOptions,
  type ICommand,
  type IConnection,
} from "@datasources/core";
import { TextCommand } from "./command.ts";

export class TextConnection extends StubConnection {
  constructor(_options?: DatasourceOptions) {
    super("text");
  }

  createCommand = (): ICommand => new TextCommand(this);

  static from = (options?: DatasourceOptions): TextConnection =>
    new TextConnection(options);


}
