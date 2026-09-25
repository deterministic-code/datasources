import type { IQuery } from "./IQuery.ts";
import type { IWhere } from "./IWhere.ts";
import type { Join, JoinKind } from "./Join.ts";
import { orderText } from "./OrderBy.ts";
import type { OrderBy } from "./OrderBy.ts";
import type { QueryStatementOptions } from "./QueryStatementOptions.ts";
import { Where } from "./Where.ts";

const joinWord: Record<Exclude<JoinKind, "cross">, string> = {
  inner: "INNER JOIN",
  left: "LEFT JOIN",
  right: "RIGHT JOIN",
  full: "FULL JOIN",
};

const joinOn = (join: Join): string => {
  if (!join.on) throw new Error("empty join");
  return new Where(join.on).getText();
};

const joinSql = (joins: Join[] = []): string =>
  joins
    .map((join) =>
      join.kind === "cross"
        ? ` CROSS JOIN ${join.table}`
        : ` ${joinWord[join.kind]} ${join.table} ON ${joinOn(join)}`,
    )
    .join("");

const withClause = (keyword: string, where?: IWhere): string =>
  where ? ` ${keyword} ${where.getText()}` : "";

const listSql = (keyword: string, items: string[] | undefined): string =>
  items && items.length > 0 ? ` ${keyword} ${items.join(", ")}` : "";

const bound = (keyword: string, value: number | undefined): string => {
  if (value === undefined) return "";
  if (value < 0) throw new Error(`negative ${keyword.toLowerCase()}`);
  return ` ${keyword} ${value}`;
};

const select = (options: QueryStatementOptions): string =>
  `SELECT ${options.distinct ? "DISTINCT " : ""}${
    options.fields.length === 0 ? "*" : options.fields.join(", ")
  } FROM ${options.table}${joinSql(options.joins)}${withClause("WHERE", options.where)}${listSql(
    "GROUP BY",
    options.groupBy,
  )}${withClause("HAVING", options.having)}${listSql(
    "ORDER BY",
    options.orderBy?.map(orderText),
  )}${bound("LIMIT", options.limit)}${bound("OFFSET", options.offset)}`;

export class QueryStatement implements IQuery {
  readonly where?: IWhere;
  readonly distinct: boolean;
  readonly table?: string;
  readonly fields: string[];
  readonly joins: Join[];
  readonly groupBy: string[];
  readonly having?: IWhere;
  readonly orderBy: OrderBy[];
  readonly limit?: number;
  readonly offset?: number;
  private readonly text: string;

  constructor(text: string, where?: IWhere);
  constructor(options: QueryStatementOptions);
  constructor(textOrOptions: string | QueryStatementOptions, where?: IWhere) {
    if (typeof textOrOptions === "string") {
      this.text = textOrOptions;
      this.where = where;
      this.distinct = false;
      this.fields = [];
      this.joins = [];
      this.groupBy = [];
      this.orderBy = [];
      return;
    }
    this.text = select(textOrOptions);
    this.where = textOrOptions.where;
    this.distinct = textOrOptions.distinct ?? false;
    this.table = textOrOptions.table;
    this.fields = textOrOptions.fields;
    this.joins = textOrOptions.joins ?? [];
    this.groupBy = textOrOptions.groupBy ?? [];
    this.having = textOrOptions.having;
    this.orderBy = textOrOptions.orderBy ?? [];
    this.limit = textOrOptions.limit;
    this.offset = textOrOptions.offset;
  }

  getText = (): string => this.text;
}
