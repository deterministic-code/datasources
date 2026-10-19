import { DropIndexStatementTextRenderer, type DropIndexStatement } from "@deterministic-code/datasources";

export class SqlServerDropIndexStatementRenderer extends DropIndexStatementTextRenderer {
  override render(statement: DropIndexStatement): string {
    return `DROP INDEX ${statement.name} ON ${statement.table}`;
  }
}
