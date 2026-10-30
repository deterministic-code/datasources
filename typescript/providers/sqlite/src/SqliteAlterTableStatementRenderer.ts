import {
  AlterTableStatementTextRenderer,
  type AlterAction,
  type Column,
} from "@deterministic-code/datasources";
import { SqliteDriver } from "./SqliteDriver.ts";

export class SqliteAlterTableStatementRenderer extends AlterTableStatementTextRenderer {
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
