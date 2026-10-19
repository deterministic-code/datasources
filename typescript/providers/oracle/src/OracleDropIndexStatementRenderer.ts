import { DropIndexStatementTextRenderer, type DropIndexStatement } from "@deterministic-code/datasources";

export class OracleDropIndexStatementRenderer extends DropIndexStatementTextRenderer {
  override render(statement: DropIndexStatement): string {
    return `DROP INDEX ${statement.name}`;
  }
}
