import { DropIndexStatementTextRenderer, type DropIndexStatement } from "@datasources/core";

export class SqlServerDropIndexStatementRenderer extends DropIndexStatementTextRenderer {
  override render(statement: DropIndexStatement): string {
    return `DROP INDEX ${statement.name} ON ${statement.table}`;
  }
}
