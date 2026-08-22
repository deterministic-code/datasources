import type { IConnection } from "./IConnection.ts";
import type { IMiddlewareHost } from "./IMiddlewareHost.ts";
import type { IReader } from "./IReader.ts";
import type { IResponse } from "./IResponse.ts";
import type { Parameter } from "./Parameter.ts";

export interface ICommand extends IMiddlewareHost {
  readonly connection: IConnection;
  executeReader(
    text?: string | null,
    parameters?: Parameter[],
  ): Promise<IReader>;
  execute(text?: string | null, parameters?: Parameter[]): Promise<IResponse>;
}
