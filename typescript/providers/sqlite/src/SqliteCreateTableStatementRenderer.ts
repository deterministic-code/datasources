import { CreateTableStatement, CreateTableStatementTextRenderer, PrimaryKey, type Column } from "@deterministic-code/datasources";
import { SqliteDriver } from "./SqliteDriver.ts";

const primaryKey = (constraint: { kind: string }): constraint is PrimaryKey =>
  constraint.kind === "primaryKey";

export class SqliteCreateTableStatementRenderer extends CreateTableStatementTextRenderer {
  override render(statement: CreateTableStatement): string {
    const identity = new Set(statement.columns.filter((column) => column.identity).map((column) => column.name));
    const constraints = statement.constraints.filter(
      (constraint) => !primaryKey(constraint) || !constraint.columns.every((column) => identity.has(column)),
    );
    return super.render(
      constraints.length === statement.constraints.length
        ? statement
        : new CreateTableStatement(statement.table, statement.columns, constraints),
    );
  }

  protected quote(name: string): string {
    return SqliteDriver.quote(name);
  }

  protected typeName(column: Column): string {
    return SqliteDriver.nativeType(column);
  }

  protected columnDefault(column: Column): string {
    return column.default == null
      ? ""
      : column.default.name == null
        ? ` DEFAULT ${SqliteDriver.nativeDefault(column)}`
        : ` CONSTRAINT ${this.quote(column.default.name)} DEFAULT ${SqliteDriver.nativeDefault(column)}`;
  }

  protected identity(column: Column): string {
    return column.identity ? " PRIMARY KEY AUTOINCREMENT" : "";
  }
}
