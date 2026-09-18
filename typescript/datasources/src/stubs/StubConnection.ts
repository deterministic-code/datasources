import type { ICommand } from "../ICommand.ts";
import type { IConnection } from "../IConnection.ts";
import type { ITransaction } from "../ITransaction.ts";
import { StubCommand } from "./StubCommand.ts";
import { notImplemented } from "./notImplemented.ts";

export class StubConnection implements IConnection {
  isOpen = false;
  constructor(readonly provider: string) {}

  createCommand = (): ICommand => new StubCommand(this.provider, this);

  open = async (): Promise<void> => notImplemented(this.provider);

  close = async (): Promise<void> => notImplemented(this.provider);

  startTransaction = async (): Promise<ITransaction> =>
    notImplemented(this.provider);
}
