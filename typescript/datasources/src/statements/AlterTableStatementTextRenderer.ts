import type { AlterAction, AlterTableStatement } from "./AlterTableStatement.ts";
import type { Column } from "./Column.ts";
import { CreateTableStatementTextRenderer } from "./CreateTableStatementTextRenderer.ts";
import type { ITableConstraint } from "./ITableConstraint.ts";

export class AlterTableStatementTextRenderer extends CreateTableStatementTextRenderer {
  renderAlter(statement: AlterTableStatement): string {
    return `ALTER TABLE ${this.quote(statement.table)} ${this.actionSql(statement.action)}`;
  }

  protected actionSql(action: AlterAction): string {
    return (
      {
        addColumn: (item: AlterAction) => `ADD COLUMN ${this.column((item as { column: Column }).column)}`,
        dropColumn: (item: AlterAction) => `DROP COLUMN ${this.quote((item as { name: string }).name)}`,
        alterColumn: (item: AlterAction) => this.alterColumn((item as { column: Column }).column),
        renameColumn: (item: AlterAction) => {
          const rename = item as { from: string; to: string };
          return `RENAME COLUMN ${this.quote(rename.from)} TO ${this.quote(rename.to)}`;
        },
        renameTable: (item: AlterAction) => `RENAME TO ${this.quote((item as { to: string }).to)}`,
        addConstraint: (item: AlterAction) =>
          `ADD ${this.constraint((item as { constraint: ITableConstraint }).constraint)}`,
        dropConstraint: (item: AlterAction) =>
          `DROP CONSTRAINT ${this.quote((item as { name: string }).name)}`,
      }[action.kind] ??
      (() => {
        throw new Error("empty alter");
      })
    )(action);
  }

  protected alterColumn(column: Column): string {
    return `ALTER COLUMN ${this.quote(column.name)} TYPE ${this.typeName(column)}${this.columnDefault(column)}${this.notNullClause(column)}`;
  }

  protected notNullClause(column: Column): string {
    return column.notNull == null
      ? ""
      : column.notNull.name == null
        ? " NOT NULL"
        : ` CONSTRAINT ${this.quote(column.notNull.name)} NOT NULL`;
  }
}
