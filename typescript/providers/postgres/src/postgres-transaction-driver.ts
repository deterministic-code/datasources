import { IsolationLevel, type IsolationLevelName } from "@datasources/core";
import type { PostgresCommandDriver } from "./postgres-command-driver.ts";

const isoSql = (isolation: IsolationLevelName): string =>
  isolation === IsolationLevel.ReadUncommitted
    ? "READ UNCOMMITTED"
    : isolation === IsolationLevel.ReadCommitted
      ? "READ COMMITTED"
      : isolation === IsolationLevel.RepeatableRead
        ? "REPEATABLE READ"
        : "SERIALIZABLE";

export class PostgresTransactionDriver {
  constructor(private readonly commands: PostgresCommandDriver) {}

  async begin(
    isolation: IsolationLevelName = IsolationLevel.Serializable,
  ): Promise<void> {
    await this.commands.exec(`BEGIN ISOLATION LEVEL ${isoSql(isolation)}`);
  }

  async commit(): Promise<void> {
    await this.commands.exec("COMMIT");
  }

  async rollback(): Promise<void> {
    await this.commands.exec("ROLLBACK");
  }
}
