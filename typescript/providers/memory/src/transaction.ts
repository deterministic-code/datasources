import {
  IsolationLevel,
  type ITransaction,
  type IsolationLevelName,
} from "@datasources/core";
import type { MemoryConnection } from "./connection.ts";

export class MemoryTransaction implements ITransaction {
  constructor(
    readonly connection: MemoryConnection,
    readonly isolation: IsolationLevelName = IsolationLevel.ReadCommitted,
  ) {}

  async commit(): Promise<void> {
    this.connection.finishCommit();
  }

  async abort(): Promise<void> {
    this.connection.finishAbort();
  }
}
