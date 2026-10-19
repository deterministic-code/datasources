import {
  AlterTableStatementTextRenderer,
  type AlterAction,
  type Column,
} from "@deterministic-code/datasources";
import { columnTypes } from "./columnTypes.ts";

export class SqliteAlterTableStatementRenderer extends AlterTableStatementTextRenderer {
  protected typeName(column: Column): string {
    return columnTypes[column.type] ?? column.type;
  }

  protected override actionSql(action: AlterAction): string {
    return (
      {
        alterColumn: () => {
          throw new Error("SQLite cannot ALTER COLUMN");
        },
        addConstraint: () => {
          throw new Error("SQLite cannot ADD CONSTRAINT");
        },
        dropConstraint: () => {
          throw new Error("SQLite cannot DROP CONSTRAINT");
        },
        addColumn: () => super.actionSql(action),
        dropColumn: () => super.actionSql(action),
        renameColumn: () => super.actionSql(action),
        renameTable: () => super.actionSql(action),
      }[action.kind] ??
      (() => {
        throw new Error("empty alter");
      })
    )();
  }
}
