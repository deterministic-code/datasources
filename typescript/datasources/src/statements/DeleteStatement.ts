import type { IDeleteStatement } from "./IDeleteStatement.ts";
import type { IWhere } from "./IWhere.ts";

const withWhere = (sql: string, where?: IWhere): string =>
  where ? `${sql} WHERE ${where.getText()}` : sql;

export class DeleteStatement implements IDeleteStatement {
  readonly where?: IWhere;
  private readonly text: string;

  constructor(table: string, where?: IWhere, raw = false) {
    this.where = where;
    this.text = raw ? table : withWhere(`DELETE FROM ${table}`, where);
  }

  getText = (): string => this.text;

  static raw = (text: string, where?: IWhere): DeleteStatement =>
    new DeleteStatement(text, where, true);
}
