import type { IWhere } from "./IWhere.ts";
import type { Join, JoinKind } from "./Join.ts";
import { orderText, type OrderBy } from "./OrderBy.ts";
import { asIndexOf, segments } from "./SelectFieldAliases.ts";
import type { SelectStatement } from "./SelectStatement.ts";
import type { StatementTextRenderer } from "./StatementTextRenderer.ts";
import { Where } from "./Where.ts";

export class SelectStatementTextRenderer {
  constructor(private readonly text: StatementTextRenderer) {}

  render(statement: SelectStatement): string {
    return `SELECT ${statement.distinct ? "DISTINCT " : ""}${
      this.fieldsSql(statement)
    } FROM ${this.quote(statement.table)}${this.joinSql(statement.joins)}${this.whereSql(statement.where)}${this.groupSql(statement.groupBy)}${this.havingSql(statement.having)}${this.orderSql(statement.orderBy)}${this.limit(statement.limit)}${this.offset(statement.offset)}`;
  }

  protected quote(name: string): string {
    return name;
  }

  protected quotePath(name: string): string {
    return name.includes("(") ? name : segments(name).map((part) => this.quote(part)).join(".");
  }

  protected fieldsSql(statement: SelectStatement): string {
    return statement.fields.length === 0
      ? "*"
      : statement.fields
          .map((field, i) => this.fieldSql(field, statement.aliases[i]))
          .join(", ");
  }

  protected fieldSql(field: string, alias?: string): string {
    const rendered = asIndexOf(field) >= 0 || field.includes("(") ? field : this.quotePath(field);
    return alias !== undefined ? `${rendered} AS ${this.quote(alias)}` : rendered;
  }

  protected whereSql(where?: IWhere): string {
    return where ? ` WHERE ${this.text.render(where)}` : "";
  }

  protected havingSql(having?: IWhere): string {
    return having ? ` HAVING ${this.text.render(having)}` : "";
  }

  protected groupSql(columns?: string[]): string {
    return !columns || columns.length === 0 ? "" : ` GROUP BY ${columns.map((column) => this.quotePath(column)).join(", ")}`;
  }

  protected orderSql(order?: OrderBy[]): string {
    return !order || order.length === 0
      ? ""
      : ` ORDER BY ${order.map((item) => `${this.quotePath(item.column)} ${orderText(item).slice(item.column.length + 1)}`).join(", ")}`;
  }

  protected limit(limit?: number): string {
    return this.bound("LIMIT", limit);
  }

  protected offset(offset?: number): string {
    return this.bound("OFFSET", offset);
  }

  protected bound(keyword: string, value?: number): string {
    if (value === undefined) return "";
    if (value < 0) throw new Error(`negative ${keyword.toLowerCase()}`);
    return ` ${keyword} ${value}`;
  }

  protected joinSql(joins?: Join[]): string {
    if (!joins || joins.length === 0) return "";
    return joins
      .map((join) =>
        join.kind === "cross"
          ? ` CROSS JOIN ${this.quote(join.table)}`
          : ` ${this.keyword(join.kind)} ${this.quote(join.table)} ON ${this.text.render(
              new Where(join.on ?? (() => { throw new Error("empty join"); })()),
            )}`,
      )
      .join("");
  }

  protected keyword(kind: JoinKind): string {
    return (
      {
        inner: "INNER JOIN",
        left: "LEFT JOIN",
        right: "RIGHT JOIN",
        full: "FULL JOIN",
        cross: "CROSS JOIN",
      } as Record<JoinKind, string>
    )[kind];
  }
}
