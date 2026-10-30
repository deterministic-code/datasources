import { DropTriggerStatementTextRenderer, type DropTriggerStatement } from "@deterministic-code/datasources";
import { OracleDriver } from "./OracleDriver.ts";

export class OracleDropTriggerStatementRenderer extends DropTriggerStatementTextRenderer {
  override render(statement: DropTriggerStatement): string {
    return `DROP TRIGGER ${OracleDriver.quote(statement.name)}`;
  }
}
