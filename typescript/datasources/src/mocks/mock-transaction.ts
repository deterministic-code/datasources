import type { IConnection } from "../IConnection.ts";
import type { ITransaction } from "../ITransaction.ts";
import { IsolationLevel } from "../IsolationLevel.ts";

export class MockTransaction implements ITransaction {
  commitCalls = 0;
  abortCalls = 0;
  commitError?: unknown;
  abortError?: unknown;
  committed = false;
  aborted = false;
  isolation: IsolationLevel = IsolationLevel.ReadCommitted;

  constructor(readonly connection: IConnection) {}

  async commit(): Promise<void> {
    this.commitCalls += 1;
    if (this.commitError) {
      throw this.commitError;
    }
    this.committed = true;
  }

  async abort(): Promise<void> {
    this.abortCalls += 1;
    if (this.abortError) {
      throw this.abortError;
    }
    this.aborted = true;
  }
}
