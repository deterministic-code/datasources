import type { IConnection } from "../IConnection.ts";
import type { ITransaction } from "../ITransaction.ts";
import { MiddlewareHost } from "../MiddlewareHost.ts";
import { notImplemented } from "./notImplemented.ts";

export class StubTransaction extends MiddlewareHost implements ITransaction {
  constructor(
    readonly provider: string,
    readonly connection: IConnection,
  ) {
    super();
  }

  async commit(): Promise<void> {
    return this.run("commit", async () => notImplemented(this.provider));
  }

  async abort(): Promise<void> {
    return this.run("abort", async () => notImplemented(this.provider));
  }
}
