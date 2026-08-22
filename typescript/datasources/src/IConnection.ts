import type { IMiddlewareHost } from "./IMiddlewareHost.ts";
import type { ITransaction } from "./ITransaction.ts";

export interface IConnection extends IMiddlewareHost {
  open(): Promise<void>;
  close(): Promise<void>;
  startTransaction(): Promise<ITransaction>;
}
