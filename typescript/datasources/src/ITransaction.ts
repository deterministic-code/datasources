import type { IMiddlewareHost } from "./IMiddlewareHost.ts";
import type { IConnection } from "./IConnection.ts";

export interface ITransaction extends IMiddlewareHost {
  readonly connection: IConnection;
  commit(): Promise<void>;
  abort(): Promise<void>;
}
