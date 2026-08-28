import {
  IsolationLevel,
  MiddlewareHost,
  type ITransaction,
  type IsolationLevelName,
} from "@datasources/core";
import type { MemoryConnection } from "./connection.ts";

export class MemoryTransaction extends MiddlewareHost implements ITransaction {
  constructor(
    readonly connection: MemoryConnection,
    readonly isolation: IsolationLevelName = IsolationLevel.Serializable,
  ) {
    super();
  }

  async commit(): Promise<void> {
    return this.run("commit", async () => {
      this.connection.finishCommit();
    });
  }

  async abort(): Promise<void> {
    return this.run("abort", async () => {
      this.connection.finishAbort();
    });
  }
}
