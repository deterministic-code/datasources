import type { IWhere } from "./IWhere.ts";
import type { Join, JoinKind } from "./Join.ts";
import { orderText, type OrderBy } from "./OrderBy.ts";
import { applySelectFieldAlias } from "../SelectFieldAliases.ts";
import type { SelectStatement } from "./SelectStatement.ts";
import type { StatementTextRenderer } from "./StatementTextRenderer.ts";
import { Where } from "./Where.ts";

export class SelectStatementTextRenderer {
  constructor(private readonly text: StatementTextRenderer) {}

  render(statement: SelectStatement): string {
    return `SELECT ${statement.distinct ? "DISTINCT " : ""}${
      statement.fields.length === 0
        ? "*"
        : statement.fields
            .map((field, i) => applySelectFieldAlias(field, statement.aliases[i]))
            .join(", ")
    } FROM ${statement.table}${this.joinSql(statement.joins)}${this.whereSql(statement.where)}${this.groupSql(statement.groupBy)}${this.havingSql(statement.having)}${this.orderSql(statement.orderBy)}${this.limit(statement.limit)}${this.offset(statement.offset)}`;
  }

  protected whereSql(where?: IWhere): string {
    return where ? ` WHERE ${this.text.render(where)}` : "";
  }

  protected havingSql(having?: IWhere): string {
    return having ? ` HAVING ${this.text.render(having)}` : "";
  }

  protected groupSql(columns?: string[]): string {
    return !columns || columns.length === 0 ? "" : ` GROUP BY ${columns.join(", ")}`;
  }

  protected orderSql(order?: OrderBy[]): string {
    return !order || order.length === 0 ? "" : ` ORDER BY ${order.map(orderText).join(", ")}`;
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
          ? ` CROSS JOIN ${join.table}`
          : ` ${this.keyword(join.kind)} ${join.table} ON ${this.text.render(
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
