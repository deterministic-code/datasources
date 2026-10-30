import { DropIndexStatementTextRenderer, type DropIndexStatement } from "@deterministic-code/datasources";
import { OracleDriver } from "./OracleDriver.ts";

export class OracleDropIndexStatementRenderer extends DropIndexStatementTextRenderer {
  override render(statement: DropIndexStatement): string {
    return `DROP INDEX ${OracleDriver.quote(statement.name)}`;
  }
}
