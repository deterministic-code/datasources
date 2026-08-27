import type { IConnection } from "../IConnection.ts";
import type { ITransaction } from "../ITransaction.ts";
import { MiddlewareHost } from "../MiddlewareHost.ts";
import { notImplemented } from "./notImplemented.ts";

export class StubConnection extends MiddlewareHost implements IConnection {
  constructor(readonly provider: string) {
    super();
  }

  open = (): Promise<void> =>
    this.run("open", async () => notImplemented(this.provider));

  close = (): Promise<void> =>
    this.run("close", async () => notImplemented(this.provider));

  startTransaction = (): Promise<ITransaction> =>
    this.run("startTransaction", async () => notImplemented(this.provider));
}
