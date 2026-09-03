import { IsolationLevel, type IsolationLevelName } from "@datasources/core";
import type { SqliteCommandDriver } from "./sqlite-command-driver.ts";

export class SqliteTransactionDriver {
  constructor(private readonly commands: SqliteCommandDriver) {}

  async begin(
    isolation: IsolationLevelName = IsolationLevel.Serializable,
  ): Promise<void> {
    this.commands.exec(
      isolation === IsolationLevel.ReadUncommitted
        ? "PRAGMA read_uncommitted = 1"
        : "PRAGMA read_uncommitted = 0",
    );
    this.commands.exec("BEGIN");
  }

  async commit(): Promise<void> {
    this.commands.exec("COMMIT");
  }

  async rollback(): Promise<void> {
    this.commands.exec("ROLLBACK");
  }
}
