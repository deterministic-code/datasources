import {
  AlterTableStatementTextRenderer,
  type AlterAction,
  type AlterTableStatement,
  type Column,
} from "@deterministic-code/datasources";
import { SqlServerDriver } from "./SqlServerDriver.ts";

export class SqlServerAlterTableStatementRenderer extends AlterTableStatementTextRenderer {
  protected quote(name: string): string {
    return SqlServerDriver.quote(name);
  }

  protected typeName(column: Column): string {
    return SqlServerDriver.nativeType(column);
  }

  protected columnDefault(column: Column): string {
    return column.default == null
      ? ""
      : column.default.name == null
        ? ` DEFAULT ${SqlServerDriver.nativeDefault(column)}`
        : ` CONSTRAINT ${this.quote(column.default.name)} DEFAULT ${SqlServerDriver.nativeDefault(column)}`;
  }
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
