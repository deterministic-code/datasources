import {
  IsolationLevel,
  type ITransaction,
  type IsolationLevelName,
} from "@datasources/core";
import type { PostgresConnection } from "./connection.ts";

export class PostgresTransaction implements ITransaction {
  constructor(
    readonly connection: PostgresConnection,
    readonly isolation: IsolationLevelName = IsolationLevel.Serializable,
  ) {}

  async commit(): Promise<void> {
    await this.connection.driver.commit();
  }

  async abort(): Promise<void> {
    await this.connection.driver.rollback();
  }
}
