import {
  StubConnection,
  type DatasourceOptions,
  type ICommand,
} from "@datasources/core";
import { YamlCommand } from "./command.ts";

export class YamlConnection extends StubConnection {
  constructor(_options?: DatasourceOptions) {
    super("yaml");
  }

  createCommand = (): ICommand => new YamlCommand(this);

  static from = (options?: DatasourceOptions): YamlConnection =>
    new YamlConnection(options);


}
