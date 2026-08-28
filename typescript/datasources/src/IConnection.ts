import type { ITransaction } from "./ITransaction.ts";
import type { IsolationLevel } from "./IsolationLevel.ts";

export interface IConnection {
  open(): Promise<void>;
  close(): Promise<void>;
  startTransaction(isolation?: IsolationLevel): Promise<ITransaction>;
}
