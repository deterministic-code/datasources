import {
  AlterTableStatementTextRenderer,
  CreateIndexStatement,
  DropIndexStatement,
  type AlterAction,
  type AlterTableStatement,
  type Column,
} from "@deterministic-code/datasources";
import { SqliteDriver } from "./SqliteDriver.ts";
import { SqliteCreateIndexStatementRenderer, SqliteDropIndexStatementRenderer } from "./SqliteQuotedRenderers.ts";

export class SqliteAlterTableStatementRenderer extends AlterTableStatementTextRenderer {
  #table = "";

  override renderAlter(statement: AlterTableStatement): string {
    this.#table = statement.table;
    const unique = this.uniqueIndexSql(statement.action);
    if (unique !== undefined) return unique;
    return super.renderAlter(statement);
  }

  private uniqueIndexSql(action: AlterAction): string | undefined {
    if (action.kind === "addConstraint" && action.constraint.kind === "unique") {
      return new SqliteCreateIndexStatementRenderer().render(
        new CreateIndexStatement(action.constraint.name, this.#table, action.constraint.columns, true),
      );
    }
    if (action.kind === "dropConstraint") {
      return new SqliteDropIndexStatementRenderer().render(new DropIndexStatement(action.name, this.#table));
    }
    return undefined;
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

  protected override actionSql(action: AlterAction): string {
    return (
      {
        alterColumn: () => {
          throw new Error("SQLite cannot ALTER COLUMN");
        },
        addConstraint: () => {
          throw new Error("SQLite cannot ADD CONSTRAINT");
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
