import { CreateTableStatementRenderer, type Column } from "@datasources/core";
import { columnTypes } from "./SqliteDriver.ts";

export class SqliteCreateTableStatementRenderer extends CreateTableStatementRenderer {
  protected typeName(column: Column): string {
    return columnTypes[column.type] ?? column.type;
  }
}
