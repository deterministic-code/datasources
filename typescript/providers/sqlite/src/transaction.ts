import {
  IsolationLevel,
  MiddlewareHost,
  type ITransaction,
  type IsolationLevelName,
} from "@datasources/core";
import type { SqliteConnection } from "./connection.ts";

export class SqliteTransaction extends MiddlewareHost implements ITransaction {
  constructor(
    readonly connection: SqliteConnection,
    readonly isolation: IsolationLevelName = IsolationLevel.Serializable,
  ) {
    super();
  }

  async commit(): Promise<void> {
    return this.run("commit", async () => {
      await this.connection.driver.commit();
    });
  }

  async abort(): Promise<void> {
    return this.run("abort", async () => {
      await this.connection.driver.rollback();
    });
  }
}
