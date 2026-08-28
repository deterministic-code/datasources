import type { IConnection } from "../IConnection.ts";
import type { ITransaction } from "../ITransaction.ts";
import { IsolationLevel } from "../IsolationLevel.ts";
import { MiddlewareHost } from "../MiddlewareHost.ts";

export class MockTransaction extends MiddlewareHost implements ITransaction {
  commitCalls = 0;
  abortCalls = 0;
  commitError?: unknown;
  abortError?: unknown;
  committed = false;
  aborted = false;
  isolation: IsolationLevel = IsolationLevel.Serializable;

  constructor(readonly connection: IConnection) {
    super();
  }

  async commit(): Promise<void> {
    return this.run("commit", async () => {
      this.commitCalls += 1;
      if (this.commitError) {
        throw this.commitError;
      }
      this.committed = true;
    });
  }

  async abort(): Promise<void> {
    return this.run("abort", async () => {
      this.abortCalls += 1;
      if (this.abortError) {
        throw this.abortError;
      }
      this.aborted = true;
    });
  }
}
