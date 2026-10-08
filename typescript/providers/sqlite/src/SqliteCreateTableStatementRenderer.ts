import { CreateTableStatement, CreateTableStatementTextRenderer, PrimaryKey, type Column } from "@datasources/core";
import { columnTypes } from "./columnTypes.ts";

export class SqliteCreateTableStatementRenderer extends CreateTableStatementTextRenderer {
  override render(statement: CreateTableStatement): string {
    const identity = new Set(statement.columns.filter((column) => column.identity).map((column) => column.name));
    const constraints = statement.constraints.filter(
      (constraint) => !(constraint instanceof PrimaryKey) || !constraint.columns.every((column) => identity.has(column)),
    );
    return super.render(
      constraints.length === statement.constraints.length
        ? statement
        : new CreateTableStatement(statement.table, statement.columns, constraints),
    );
  }

  protected typeName(column: Column): string {
    return columnTypes[column.type] ?? column.type;
  }

  protected identity(column: Column): string {
    return column.identity ? " PRIMARY KEY AUTOINCREMENT" : "";
  }
}
