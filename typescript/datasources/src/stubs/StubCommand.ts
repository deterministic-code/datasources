import type { ICommand } from "../ICommand.ts";
import type { IConnection } from "../IConnection.ts";
import type { Cells } from "../converters/Cell.ts";
import type { IReader } from "../IReader.ts";
import type { Response } from "../Response.ts";
import type { Parameter } from "../Parameter.ts";
import { CommandMiddlewareHost } from "../CommandMiddlewareHost.ts";
import { notImplemented } from "./notImplemented.ts";

export class StubCommand extends CommandMiddlewareHost implements ICommand {
  constructor(
    readonly provider: string,
    readonly connection: IConnection,
  ) {
    super();
  }

  executeReader = (
    text: string,
    parameters?: Parameter[],
  ): Promise<IReader<Cells>> =>
    this.run(text, parameters, async () => notImplemented(this.provider));

  execute = (
    text: string,
    parameters?: Parameter[],
  ): Promise<Response> =>
    this.run(text, parameters, async () => notImplemented(this.provider));
}
