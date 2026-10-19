import { DropTriggerStatementTextRenderer, type DropTriggerStatement } from "@deterministic-code/datasources";

export class OracleDropTriggerStatementRenderer extends DropTriggerStatementTextRenderer {
  override render(statement: DropTriggerStatement): string {
    return `DROP TRIGGER ${statement.name}`;
  }
}
