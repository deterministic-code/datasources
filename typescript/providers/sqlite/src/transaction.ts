import {
  IsolationLevel,
  type ITransaction,
  type IsolationLevelName,
} from "@datasources/core";
import type { SqliteConnection } from "./connection.ts";

export class SqliteTransaction implements ITransaction {
  constructor(
    readonly connection: SqliteConnection,
    readonly isolation: IsolationLevelName = IsolationLevel.Serializable,
  ) {}

  async commit(): Promise<void> {
    await this.connection.driver.commit();
  }

  async abort(): Promise<void> {
    await this.connection.driver.rollback();
  }
}
