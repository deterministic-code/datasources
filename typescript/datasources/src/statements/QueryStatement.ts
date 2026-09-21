import type { IQuery } from "./IQuery.ts";
import type { IWhere } from "./IWhere.ts";
import type { Join, JoinKind } from "./Join.ts";

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

const select = (table: string, fields: string[], joins: Join[] | undefined, where?: IWhere): string =>
  withWhere(
    `SELECT ${fields.length === 0 ? "*" : fields.join(", ")} FROM ${table}${joinSql(joins)}`,
    where,
  );

export class QueryStatement implements IQuery {
  readonly where?: IWhere;
  private readonly text: string;

  constructor(text: string, where?: IWhere);
  constructor(table: string, fields: string[], where?: IWhere, joins?: Join[]);
  constructor(
    tableOrText: string,
    fieldsOrWhere?: string[] | IWhere,
    where?: IWhere,
    joins?: Join[],
  ) {
    if (Array.isArray(fieldsOrWhere)) {
      this.text = select(tableOrText, fieldsOrWhere, joins, where);
      this.where = where;
      return;
    }
    this.text = tableOrText;
    this.where = fieldsOrWhere;
  }

  getText = (): string => this.text;
}
