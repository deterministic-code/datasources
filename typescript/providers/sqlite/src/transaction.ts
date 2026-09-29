import {
  IsolationLevel,
  type ITransaction,
  type IsolationLevelName,
} from "@datasources/core";
import type { SqliteConnection } from "./connection.ts";

export class SqliteTransaction implements ITransaction {
  constructor(
    readonly connection: SqliteConnection,
    readonly isolation: IsolationLevelName = IsolationLevel.ReadCommitted,
  ) {}

  async commit(): Promise<void> {
    this.connection.exec("COMMIT");
  }

  async abort(): Promise<void> {
    this.connection.exec("ROLLBACK");
  }
}
