import { DropTriggerStatementTextRenderer, type DropTriggerStatement } from "@datasources/core";

export class PostgresDropTriggerStatementRenderer extends DropTriggerStatementTextRenderer {
  override render(statement: DropTriggerStatement): string {
    return `DROP TRIGGER IF EXISTS ${statement.name} ON ${statement.table}`;
  }
}
