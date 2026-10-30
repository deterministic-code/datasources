import { DropIndexStatementTextRenderer, type DropIndexStatement } from "@deterministic-code/datasources";
import { SqlServerDriver } from "./SqlServerDriver.ts";

export class SqlServerDropIndexStatementRenderer extends DropIndexStatementTextRenderer {
  override render(statement: DropIndexStatement): string {
    return `DROP INDEX ${SqlServerDriver.quote(statement.name)} ON ${SqlServerDriver.quote(statement.table)}`;
  }
}
