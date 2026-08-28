import type { ICommand } from "./ICommand.ts";
import type { ITransaction } from "./ITransaction.ts";
import type { IsolationLevel } from "./IsolationLevel.ts";

export interface IConnection {
  readonly provider: string;
  createCommand(): ICommand;
  open(): Promise<void>;
  close(): Promise<void>;
  startTransaction(isolation?: IsolationLevel): Promise<ITransaction>;
}
