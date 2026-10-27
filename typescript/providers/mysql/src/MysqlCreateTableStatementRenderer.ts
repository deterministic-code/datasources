import { CreateTableStatementTextRenderer, type Column, type NotNull } from "@deterministic-code/datasources";
import { columnTypes } from "./columnTypes.ts";

export class MysqlCreateTableStatementRenderer extends CreateTableStatementTextRenderer {
  protected typeName(column: Column): string {
    return columnTypes[column.type] ?? column.type;
  }

  protected columnDefault(column: Column): string {
    return column.default == null ? "" : ` DEFAULT ${column.default.value}`;
  }

  protected notNullSql(value?: NotNull): string {
    return value == null ? "" : " NOT NULL";
  }

  protected identity(column: Column): string {
    return column.identity ? " AUTO_INCREMENT" : "";
  }
}
