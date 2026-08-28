import type { IConnection } from "../IConnection.ts";
import type { ITransaction } from "../ITransaction.ts";
import { notImplemented } from "./notImplemented.ts";

export class StubConnection implements IConnection {
  constructor(readonly provider: string) {}

  open = async (): Promise<void> => notImplemented(this.provider);

  close = async (): Promise<void> => notImplemented(this.provider);

  startTransaction = async (): Promise<ITransaction> =>
    notImplemented(this.provider);
}
