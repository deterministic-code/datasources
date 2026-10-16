import { DropIndexStatementTextRenderer, type DropIndexStatement } from "@datasources/core";

export class OracleDropIndexStatementRenderer extends DropIndexStatementTextRenderer {
  override render(statement: DropIndexStatement): string {
    return `DROP INDEX ${statement.name}`;
  }
}
