import { CreateTableStatementTextRenderer, type Column } from "@datasources/core";
import { columnTypes } from "./columnTypes.ts";

export class SqliteCreateTableStatementRenderer extends CreateTableStatementTextRenderer {
  protected typeName(column: Column): string {
    return columnTypes[column.type] ?? column.type;
  }
}
