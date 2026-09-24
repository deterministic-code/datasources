import type { IQuery } from "./IQuery.ts";
import type { IWhere } from "./IWhere.ts";
import type { Join, JoinKind } from "./Join.ts";
import { orderText } from "./OrderBy.ts";
import type { QueryStatementOptions } from "./QueryStatementOptions.ts";

const joinKeyword: Record<JoinKind, string> = {
  inner: "INNER JOIN",
  left: "LEFT JOIN",
  right: "RIGHT JOIN",
};

const joinSql = (joins: Join[] = []): string =>
  joins
    .map((join) => ` ${joinKeyword[join.kind]} ${join.table} ON ${join.left} = ${join.right}`)
    .join("");

const withWhere = (sql: string, where?: IWhere): string =>
  where ? `${sql} WHERE ${where.getText()}` : sql;

const listSql = (keyword: string, items: string[] | undefined): string =>
  items && items.length > 0 ? ` ${keyword} ${items.join(", ")}` : "";

const limitSql = (limit: number | undefined): string => {
  if (limit === undefined) return "";
  if (limit < 0) throw new Error("negative limit");
  return ` LIMIT ${limit}`;
};

const select = (options: QueryStatementOptions): string =>
  `${withWhere(
    `SELECT ${options.fields.length === 0 ? "*" : options.fields.join(", ")} FROM ${options.table}${joinSql(options.joins)}`,
    options.where,
  )}${listSql("GROUP BY", options.groupBy)}${listSql("ORDER BY", options.orderBy?.map(orderText))}${limitSql(options.limit)}`;

export class QueryStatement implements IQuery {
  readonly where?: IWhere;
  private readonly text: string;

  constructor(text: string, where?: IWhere);
  constructor(options: QueryStatementOptions);
  constructor(textOrOptions: string | QueryStatementOptions, where?: IWhere) {
    if (typeof textOrOptions === "string") {
      this.text = textOrOptions;
      this.where = where;
      return;
    }
    this.text = select(textOrOptions);
    this.where = textOrOptions.where;
  }

  getText = (): string => this.text;
}
