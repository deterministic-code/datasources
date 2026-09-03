import { IsolationLevel, type IsolationLevelName } from "@datasources/core";
import type { MysqlConnectionDriver } from "./mysql-connection-driver.ts";

const isoSql = (isolation: IsolationLevelName): string =>
  isolation === IsolationLevel.ReadUncommitted
    ? "READ UNCOMMITTED"
    : isolation === IsolationLevel.ReadCommitted
      ? "READ COMMITTED"
      : "REPEATABLE READ";

export class MysqlTransactionDriver {
  constructor(private readonly connection: MysqlConnectionDriver) {}

  async begin(
    isolation: IsolationLevelName = IsolationLevel.Serializable,
  ): Promise<void> {
    await this.connection.require().query(
      `SET TRANSACTION ISOLATION LEVEL ${isoSql(isolation)}`,
    );
    await this.connection.require().beginTransaction();
  }

  async commit(): Promise<void> {
    await this.connection.require().commit();
  }

  async rollback(): Promise<void> {
    await this.connection.require().rollback();
  }
}
