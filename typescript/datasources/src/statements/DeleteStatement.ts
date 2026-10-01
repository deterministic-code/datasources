import type { IDeleteStatement } from "./IDeleteStatement.ts";
import type { IWhere } from "./IWhere.ts";

const withWhere = (sql: string, where?: IWhere): string =>
  where ? `${sql} WHERE ${where.getText()}` : sql;

export class DeleteStatement implements IDeleteStatement {
  readonly table?: string;
  readonly where?: IWhere;
  private readonly text?: string;

  constructor(table: string, where?: IWhere, raw = false) {
    this.where = where;
    if (raw) this.text = table;
    else this.table = table;
  }

  getText = (): string => this.text ?? withWhere(`DELETE FROM ${this.table}`, this.where);

  static raw = (text: string, where?: IWhere): DeleteStatement =>
    new DeleteStatement(text, where, true);
}
