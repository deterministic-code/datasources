import type { ICommand } from "../ICommand.ts";
import type { IConnection } from "../IConnection.ts";
import type { IReader } from "../IReader.ts";
import type { IResponse } from "../IResponse.ts";
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
    text?: string | null,
    parameters?: Parameter[],
  ): Promise<IReader> =>
    this.run(text, parameters, async () => notImplemented(this.provider));

  execute = (
    text?: string | null,
    parameters?: Parameter[],
  ): Promise<IResponse> =>
    this.run(text, parameters, async () => notImplemented(this.provider));
}
