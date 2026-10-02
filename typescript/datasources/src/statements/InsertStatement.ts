import type { Field } from "../builders/Field.ts";
import type { IInsertStatement } from "./IInsertStatement.ts";
import type { IStatementVisitor } from "./IStatementVisitor.ts";

export class InsertStatement implements IInsertStatement {
  readonly table: string;
  readonly fields: Field[];

  constructor(table: string, fields: Field[]) {
    this.table = table;
    this.fields = fields;
  }

  accept = (visitor: IStatementVisitor): string => visitor.visit(this);
}
