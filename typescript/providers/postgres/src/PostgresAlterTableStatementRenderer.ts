import {
  AlterTableStatementTextRenderer,
  type AlterAction,
  type Column,
} from "@deterministic-code/datasources";
import { columnTypes } from "./columnTypes.ts";

export class PostgresAlterTableStatementRenderer extends AlterTableStatementTextRenderer {
  protected typeName(column: Column): string {
    return columnTypes[column.type] ?? column.type;
  }

  protected override actionSql(action: AlterAction): string {
    return action.kind === "alterColumn" ? this.postgresAlter(action.column) : super.actionSql(action);
  }

  private postgresAlter(column: Column): string {
    const name = this.quote(column.name);
    const defaultSql =
      column.default == null
        ? `ALTER COLUMN ${name} DROP DEFAULT`
        : `ALTER COLUMN ${name} SET DEFAULT ${column.default.value}`;
    const nullSql =
      column.notNull == null
        ? `ALTER COLUMN ${name} DROP NOT NULL`
        : `ALTER COLUMN ${name} SET NOT NULL`;
    return `ALTER COLUMN ${name} TYPE ${this.typeName(column)}, ${defaultSql}, ${nullSql}`;
  }
}
