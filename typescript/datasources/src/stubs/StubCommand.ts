import type { ICommand } from "../ICommand.ts";
import type { IConnection } from "../IConnection.ts";
import type { IReader } from "../IReader.ts";
import type { IResponse } from "../IResponse.ts";
import type { Parameter } from "../Parameter.ts";
import { MiddlewareHost } from "../MiddlewareHost.ts";
import { notImplemented } from "./notImplemented.ts";

export class StubCommand extends MiddlewareHost implements ICommand {
  constructor(
    readonly provider: string,
    readonly connection: IConnection,
  ) {
    super();
  }

  async executeReader(
    text?: string | null,
    parameters?: Parameter[],
  ): Promise<IReader> {
    return this.run(
      "executeReader",
      async () => notImplemented(this.provider),
      { text, parameters },
    );
  }

  async execute(
    text?: string | null,
    parameters?: Parameter[],
  ): Promise<IResponse> {
    return this.run(
      "execute",
      async () => notImplemented(this.provider),
      { text, parameters },
    );
  }
}
