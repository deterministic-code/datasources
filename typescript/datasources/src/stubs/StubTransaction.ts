import type { IConnection } from "../IConnection.ts";
import type { ITransaction } from "../ITransaction.ts";
import { IsolationLevel } from "../IsolationLevel.ts";
import { notImplemented } from "./notImplemented.ts";

export class StubTransaction implements ITransaction {
  readonly isolation = IsolationLevel.Serializable;

  constructor(
    readonly provider: string,
    readonly connection: IConnection,
  ) {}

  commit = async (): Promise<void> => notImplemented(this.provider);

  abort = async (): Promise<void> => notImplemented(this.provider);
}
