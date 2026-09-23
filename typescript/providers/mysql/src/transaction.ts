import {
  IsolationLevel,
  type ITransaction,
  type IsolationLevelName,
} from "@datasources/core";
import type { MysqlConnection } from "./connection.ts";

export class MysqlTransaction implements ITransaction {
  constructor(
    readonly connection: MysqlConnection,
    readonly isolation: IsolationLevelName = IsolationLevel.Serializable,
  ) {}

  async commit(): Promise<void> {
    await this.connection.require().commit();
  }

  async abort(): Promise<void> {
    await this.connection.require().rollback();
  }
}
