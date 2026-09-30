import type { ICommand } from "../ICommand.ts";
import type { IConnection } from "../IConnection.ts";
import type { Cells } from "../converters/Cell.ts";
import type { IReader } from "../IReader.ts";
import type { Response } from "../Response.ts";
import type { Parameter } from "../Parameter.ts";
import { CommandMiddlewareHost } from "../CommandMiddlewareHost.ts";
import { Native } from "../statements/Native.ts";
import { notImplemented } from "./notImplemented.ts";

export class StubCommand extends CommandMiddlewareHost implements ICommand {
  constructor(
    readonly provider: string,
    readonly connection: IConnection,
  ) {
    super();
  }

  executeReaderNative = (
    text: Native,
    parameters?: Parameter[],
  ): Promise<IReader<Cells>> =>
    this.run(text.text, parameters, async () => notImplemented(this.provider));

  executeNative = (
    text: Native,
    parameters?: Parameter[],
  ): Promise<Response> =>
    this.run(text.text, parameters, async () => notImplemented(this.provider));
}
