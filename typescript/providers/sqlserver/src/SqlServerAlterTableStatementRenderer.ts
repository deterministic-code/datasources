import {
  AlterTableStatementTextRenderer,
  type AlterAction,
  type AlterTableStatement,
} from "@deterministic-code/datasources";

export class SqlServerAlterTableStatementRenderer extends AlterTableStatementTextRenderer {
  override renderAlter(statement: AlterTableStatement): string {
    return statement.action.kind === "renameColumn"
      ? `EXEC sp_rename '${statement.table}.${statement.action.from}', '${statement.action.to}', 'COLUMN'`
      : statement.action.kind === "renameTable"
        ? `EXEC sp_rename '${statement.table}', '${statement.action.to}'`
        : super.renderAlter(statement);
  }

  protected override actionSql(action: AlterAction): string {
    return action.kind === "alterColumn"
      ? `ALTER COLUMN ${this.quote(action.column.name)} ${this.typeName(action.column)}${this.notNullClause(action.column)}`
      : super.actionSql(action);
  }
}
