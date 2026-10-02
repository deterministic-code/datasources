import type { Field } from "../builders/Field.ts";
import type { Parameter } from "../Parameter.ts";
import { DefaultStatementTextRenderer } from "./DefaultStatementTextRenderer.ts";
import type { IStatementVisitor } from "./IStatementVisitor.ts";
import type { IUpdateStatement } from "./IUpdateStatement.ts";
import type { IWhere } from "./IWhere.ts";
import { UpdateStatementTextRenderer } from "./UpdateStatementTextRenderer.ts";

export class UpdateStatement implements IUpdateStatement {
  readonly table: string;
  readonly fields: Field[];
  readonly where?: IWhere;

  constructor(table: string, fields: Field[], where?: IWhere) {
    this.table = table;
    this.fields = fields;
    this.where = where;
  }

  accept = (visitor: IStatementVisitor): string => visitor.visit(this);

  get parameters(): Parameter[] {
    return new UpdateStatementTextRenderer(DefaultStatementTextRenderer.instance)
      .compose(
        `UPDATE ${this.table} SET ${this.fields.map((item) => `${item.name} = $${item.name}`).join(", ")}`,
        this.where,
        this.fields,
      ).parameters;
  }

  static parameterList = (fields: Field[], where?: IWhere): Parameter[] =>
    new UpdateStatementTextRenderer(DefaultStatementTextRenderer.instance).compose("", where, fields)
      .parameters;
}
