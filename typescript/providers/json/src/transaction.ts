import {
  IsolationLevel,
  type ITransaction,
  type IsolationLevelName,
} from "@datasources/core";
import type { JsonConnection } from "./connection.ts";

export class JsonTransaction implements ITransaction {
  constructor(
    readonly connection: JsonConnection,
    readonly isolation: IsolationLevelName = IsolationLevel.ReadCommitted,
  ) {}

  async commit(): Promise<void> {
    this.connection.finishCommit();
    if (this.connection.path) {
      await this.connection.fileStore.write(
        this.connection.path,
        JSON.stringify(this.connection.data),
      );
    }
  }

  async abort(): Promise<void> {
    this.connection.finishAbort();
  }
}
