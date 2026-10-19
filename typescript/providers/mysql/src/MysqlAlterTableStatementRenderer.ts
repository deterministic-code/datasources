import {
  AlterTableStatementTextRenderer,
  type AlterAction,
  type Column,
} from "@deterministic-code/datasources";
import { columnTypes } from "./columnTypes.ts";

export class MysqlAlterTableStatementRenderer extends AlterTableStatementTextRenderer {
  protected typeName(column: Column): string {
    return columnTypes[column.type] ?? column.type;
  }

  protected identity(column: Column): string {
    return column.identity ? " AUTO_INCREMENT" : "";
  }

  protected override actionSql(action: AlterAction): string {
    return action.kind === "alterColumn" ? `MODIFY COLUMN ${this.column(action.column)}` : super.actionSql(action);
  }
}
