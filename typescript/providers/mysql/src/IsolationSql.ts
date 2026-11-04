import { IsolationLevel, type IsolationLevelName } from "@deterministic-code/datasources";

export class IsolationSql {
  static of(isolation: IsolationLevelName): string {
    return isolation === IsolationLevel.ReadUncommitted
      ? "READ UNCOMMITTED"
      : isolation === IsolationLevel.ReadCommitted
        ? "READ COMMITTED"
        : "REPEATABLE READ";
  }
}
