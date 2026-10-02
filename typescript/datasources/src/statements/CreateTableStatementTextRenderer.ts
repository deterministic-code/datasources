import type { CheckConstraint } from "./CheckConstraint.ts";
import type { Column } from "./Column.ts";
import type { CreateTableStatement } from "./CreateTableStatement.ts";
import type { ForeignKey, ReferentialAction } from "./ForeignKey.ts";
import type { ITableConstraint } from "./ITableConstraint.ts";
import type { NotNull } from "./NotNull.ts";
import type { PrimaryKey } from "./PrimaryKey.ts";
import type { UniqueConstraint } from "./UniqueConstraint.ts";

const actionKeyword: Record<ReferentialAction, string> = {
  cascade: "CASCADE",
  setNull: "SET NULL",
  setDefault: "SET DEFAULT",
  restrict: "RESTRICT",
  noAction: "NO ACTION",
};

export class CreateTableStatementTextRenderer {
  render(statement: CreateTableStatement): string {
    return `CREATE TABLE ${this.quote(statement.table!)} (${this.parts(statement).join(", ")})${this.suffix()}`;
  }

  protected quote(name: string): string {
    return name;
  }

  protected suffix(): string {
    return "";
  }

  protected typeName(column: Column): string {
    return column.type;
  }

  protected columnDefault(column: Column): string {
    return column.default == null
      ? ""
      : column.default.name == null
        ? ` DEFAULT ${column.default.value}`
        : ` CONSTRAINT ${this.quote(column.default.name)} DEFAULT ${column.default.value}`;
  }

  protected identity(_column: Column): string {
    return "";
  }

  protected column(column: Column): string {
    return `${this.quote(column.name)} ${this.typeName(column)}${this.identity(column)}${this.columnDefault(column)}${this.notNullSql(column.notNull)}`;
  }

  protected constraint(constraint: ITableConstraint): string {
    return (
      {
        primaryKey: (item: PrimaryKey) =>
          `CONSTRAINT ${this.quote(item.name)} PRIMARY KEY (${this.quoted(item.columns)})`,
        foreignKey: (item: ForeignKey) => this.foreignKey(item),
        unique: (item: UniqueConstraint) =>
          `CONSTRAINT ${this.quote(item.name)} UNIQUE (${this.quoted(item.columns)})`,
        check: (item: CheckConstraint) =>
          `CONSTRAINT ${this.quote(item.name)} CHECK (${item.expression})`,
      } as Record<ITableConstraint["kind"], (item: ITableConstraint) => string>
    )[constraint.kind]?.(constraint) ??
      (() => {
        throw new Error("empty constraint");
      })();
  }

  private parts(statement: CreateTableStatement): string[] {
    return statement.constraints.length === 0
      ? statement.columns.map((column) => this.column(column))
      : [
          ...statement.columns.map((column) => this.column(column)),
          ...statement.constraints.map((constraint) => this.constraint(constraint)),
        ];
  }

  private notNullSql(value?: NotNull): string {
    return value == null ? "" : value.name == null ? " NOT NULL" : ` CONSTRAINT ${this.quote(value.name)} NOT NULL`;
  }

  private foreignKey(fk: ForeignKey): string {
    return `CONSTRAINT ${this.quote(fk.name)} FOREIGN KEY (${this.quoted(fk.columns)}) REFERENCES ${this.quote(fk.referencesTable)} (${this.quoted(fk.referencesColumns)})${this.actionSql("DELETE", fk.onDelete)}${this.actionSql("UPDATE", fk.onUpdate)}`;
  }

  private quoted(names: string[]): string {
    return names.map((name) => this.quote(name)).join(", ");
  }

  private actionSql(kind: string, action?: ReferentialAction): string {
    return action == null ? "" : ` ON ${kind} ${actionKeyword[action]}`;
  }
}
