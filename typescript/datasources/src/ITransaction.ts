import type { IConnection } from "./IConnection.ts";
import type { IsolationLevel } from "./IsolationLevel.ts";

export interface ITransaction {
  readonly connection: IConnection;
  readonly isolation: IsolationLevel;
  commit(): Promise<void>;
  abort(): Promise<void>;
}
