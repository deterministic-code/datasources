import { DropTriggerStatementTextRenderer, type DropTriggerStatement } from "@datasources/core";

export class OracleDropTriggerStatementRenderer extends DropTriggerStatementTextRenderer {
  override render(statement: DropTriggerStatement): string {
    return `DROP TRIGGER ${statement.name}`;
  }
}
