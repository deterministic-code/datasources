import { MiddlewareHost, type ITransaction } from "@datasources/core";
import type { SqliteConnection } from "./connection.ts";

export class SqliteTransaction extends MiddlewareHost implements ITransaction {
  constructor(readonly connection: SqliteConnection) {
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
