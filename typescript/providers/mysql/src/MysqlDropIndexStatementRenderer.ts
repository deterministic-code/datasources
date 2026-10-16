import { DropIndexStatementTextRenderer, type DropIndexStatement } from "@datasources/core";

export class MysqlDropIndexStatementRenderer extends DropIndexStatementTextRenderer {
  override render(statement: DropIndexStatement): string {
    return `DROP INDEX ${statement.name} ON ${statement.table}`;
  }
}
