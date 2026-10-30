import { DropTableStatementTextRenderer, type DropTableStatement } from "@deterministic-code/datasources";
import { OracleDriver } from "./OracleDriver.ts";

export class OracleDropTableStatementRenderer extends DropTableStatementTextRenderer {
  override render(statement: DropTableStatement): string {
    return `BEGIN EXECUTE IMMEDIATE 'DROP TABLE ${OracleDriver.quote(statement.table)} CASCADE CONSTRAINTS'; EXCEPTION WHEN OTHERS THEN IF SQLCODE != -942 THEN RAISE; END IF; END;
/`;
  }
}
