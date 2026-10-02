import { CreateTableStatementTextRenderer, type Column } from "@datasources/core";
import { columnTypes } from "./columnTypes.ts";

export class MysqlCreateTableStatementRenderer extends CreateTableStatementTextRenderer {
  protected typeName(column: Column): string {
    return columnTypes[column.type] ?? column.type;
  }

  protected identity(column: Column): string {
    return column.identity ? " AUTO_INCREMENT" : "";
  }
}
