import { DropTableStatementTextRenderer, type DropTableStatement } from "@datasources/core";

export class OracleDropTableStatementRenderer extends DropTableStatementTextRenderer {
  override render(statement: DropTableStatement): string {
    return `BEGIN EXECUTE IMMEDIATE 'DROP TABLE ${statement.table} CASCADE CONSTRAINTS'; EXCEPTION WHEN OTHERS THEN IF SQLCODE != -942 THEN RAISE; END IF; END;
/`;
  }
}
