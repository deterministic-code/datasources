import {
  AlterTableStatementTextRenderer,
  type AlterAction,
  type Column,
} from "@deterministic-code/datasources";
import { PostgresDriver } from "./PostgresDriver.ts";

export class PostgresAlterTableStatementRenderer extends AlterTableStatementTextRenderer {
  protected quote(name: string): string {
    return PostgresDriver.quote(name);
  }

  protected typeName(column: Column): string {
    return PostgresDriver.nativeType(column);
  }

  protected columnDefault(column: Column): string {
    return column.default == null
      ? ""
      : column.default.name == null
        ? ` DEFAULT ${PostgresDriver.nativeDefault(column)}`
        : ` CONSTRAINT ${this.quote(column.default.name)} DEFAULT ${PostgresDriver.nativeDefault(column)}`;
  }

  protected override actionSql(action: AlterAction): string {
    return action.kind === "alterColumn" ? this.postgresAlter(action.column) : super.actionSql(action);
  }

  private postgresAlter(column: Column): string {
    const name = this.quote(column.name);
    const defaultSql =
      column.default == null
        ? `ALTER COLUMN ${name} DROP DEFAULT`
        : `ALTER COLUMN ${name} SET DEFAULT ${PostgresDriver.nativeDefault(column)}`;
    const nullSql =
      column.notNull == null
        ? `ALTER COLUMN ${name} DROP NOT NULL`
        : `ALTER COLUMN ${name} SET NOT NULL`;
    return `ALTER COLUMN ${name} TYPE ${this.typeName(column)}, ${defaultSql}, ${nullSql}`;
  }
}
