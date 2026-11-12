import type { Field } from "../builders/Field.ts";
import type { IInsertStatement } from "./IInsertStatement.ts";
import type { IStatementVisitor } from "./IStatementVisitor.ts";

export class InsertStatement implements IInsertStatement {
  readonly table: string;
  readonly fields: Field[];
  readonly returning?: string;

  constructor(table: string, fields: Field[], returning?: string) {
    this.table = table;
    this.fields = fields;
    if (returning !== undefined) this.returning = returning;
  }

  accept = (visitor: IStatementVisitor): string => visitor.visit(this);
}
