import type { IMiddlewareHost } from "./IMiddlewareHost.ts";
import type { ITransaction } from "./ITransaction.ts";
import type { IsolationLevel } from "./IsolationLevel.ts";

export interface IConnection extends IMiddlewareHost {
  open(): Promise<void>;
  close(): Promise<void>;
  startTransaction(isolation?: IsolationLevel): Promise<ITransaction>;
}
