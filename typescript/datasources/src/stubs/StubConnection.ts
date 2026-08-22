import type { IConnection } from "../IConnection.ts";
import type { ITransaction } from "../ITransaction.ts";
import { MiddlewareHost } from "../MiddlewareHost.ts";
import { notImplemented } from "./notImplemented.ts";

export class StubConnection extends MiddlewareHost implements IConnection {
  constructor(readonly provider: string) {
    super();
  }

  async open(): Promise<void> {
    return this.run("open", async () => notImplemented(this.provider));
  }

  async close(): Promise<void> {
    return this.run("close", async () => notImplemented(this.provider));
  }

  async startTransaction(): Promise<ITransaction> {
    return this.run("startTransaction", async () => notImplemented(this.provider));
  }
}
