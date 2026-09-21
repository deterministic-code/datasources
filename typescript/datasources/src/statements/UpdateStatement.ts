import type { Field } from "../builders/Field.ts";
import type { IUpdateStatement } from "./IUpdateStatement.ts";
import type { IWhere } from "./IWhere.ts";

const withWhere = (sql: string, where?: IWhere): string =>
  where ? `${sql} WHERE ${where.getText()}` : sql;

export class UpdateStatement implements IUpdateStatement {
  readonly where?: IWhere;
  private readonly text: string;

  constructor(text: string, where?: IWhere);
  constructor(table: string, fields: Field[], where?: IWhere);
  constructor(tableOrText: string, fieldsOrWhere?: Field[] | IWhere, where?: IWhere) {
    if (Array.isArray(fieldsOrWhere)) {
      this.text = withWhere(
        `UPDATE ${tableOrText} SET ${fieldsOrWhere.map((field) => `${field.name} = $${field.name}`).join(", ")}`,
        where,
      );
      this.where = where;
      return;
    }
    this.text = tableOrText;
    this.where = fieldsOrWhere;
  }

  getText = (): string => this.text;
}
