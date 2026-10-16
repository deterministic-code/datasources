import { AlterTableStatementTextRenderer, type AlterAction } from "@datasources/core";

export class OracleAlterTableStatementRenderer extends AlterTableStatementTextRenderer {
  protected override actionSql(action: AlterAction): string {
    return action.kind === "addColumn"
      ? `ADD (${this.column(action.column)})`
      : action.kind === "alterColumn"
        ? `MODIFY (${this.column(action.column)})`
        : super.actionSql(action);
  }
}
