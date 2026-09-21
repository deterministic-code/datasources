import type { Column } from "./Column.ts";
import type { ColumnDefault } from "./ColumnDefault.ts";
import type { ICreateTableStatement } from "./ICreateTableStatement.ts";
import type { ITableConstraint } from "./ITableConstraint.ts";
import type { NotNull } from "./NotNull.ts";
import type { CheckConstraint } from "./CheckConstraint.ts";
import { type ForeignKey, type ReferentialAction } from "./ForeignKey.ts";
import type { PrimaryKey } from "./PrimaryKey.ts";
import type { UniqueConstraint } from "./UniqueConstraint.ts";

const defaultSql = (value?: ColumnDefault): string =>
  value == null ? "" : value.name == null ? ` DEFAULT ${value.value}` : ` CONSTRAINT ${value.name} DEFAULT ${value.value}`;

const notNullSql = (value?: NotNull): string =>
  value == null ? "" : value.name == null ? " NOT NULL" : ` CONSTRAINT ${value.name} NOT NULL`;

const columnSql = (column: Column): string =>
  `${column.name} ${column.type}${defaultSql(column.default)}${notNullSql(column.notNull)}`;

const actionKeyword: Record<ReferentialAction, string> = {
  cascade: "CASCADE",
  setNull: "SET NULL",
  setDefault: "SET DEFAULT",
  restrict: "RESTRICT",
  noAction: "NO ACTION",
};

const actionSql = (kind: string, action?: ReferentialAction): string =>
  action == null ? "" : ` ON ${kind} ${actionKeyword[action]}`;

const foreignKeySql = (fk: ForeignKey): string =>
  `CONSTRAINT ${fk.name} FOREIGN KEY (${fk.columns.join(", ")}) REFERENCES ${fk.referencesTable} (${fk.referencesColumns.join(", ")})${actionSql("DELETE", fk.onDelete)}${actionSql("UPDATE", fk.onUpdate)}`;

const constraintSql = (constraint: ITableConstraint): string =>
  (
    {
      primaryKey: (c: PrimaryKey) => `CONSTRAINT ${c.name} PRIMARY KEY (${c.columns.join(", ")})`,
      foreignKey: foreignKeySql,
      unique: (c: UniqueConstraint) => `CONSTRAINT ${c.name} UNIQUE (${c.columns.join(", ")})`,
      check: (c: CheckConstraint) => `CONSTRAINT ${c.name} CHECK (${c.expression})`,
    } as Record<ITableConstraint["kind"], (c: ITableConstraint) => string>
  )[constraint.kind]?.(constraint) ??
  (() => {
    throw new Error("empty constraint");
  })();

const parts = (columns: Column[], constraints?: ITableConstraint[]): string[] =>
  constraints == null || constraints.length === 0
    ? columns.map(columnSql)
    : [...columns.map(columnSql), ...constraints.map(constraintSql)];

export class CreateTableStatement implements ICreateTableStatement {
  private readonly text: string;

  constructor(text: string);
  constructor(table: string, columns: Column[], constraints?: ITableConstraint[]);
  constructor(tableOrText: string, columns?: Column[], constraints?: ITableConstraint[]) {
    this.text = columns
      ? `CREATE TABLE ${tableOrText} (${parts(columns, constraints).join(", ")})`
      : tableOrText;
  }

  getText = (): string => this.text;
}
