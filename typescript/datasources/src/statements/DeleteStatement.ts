import type { IDeleteStatement } from "./IDeleteStatement.ts";
import type { IStatementVisitor } from "./IStatementVisitor.ts";
import type { IWhere } from "./IWhere.ts";

export class DeleteStatement implements IDeleteStatement {
  readonly table: string;
  readonly where?: IWhere;

  constructor(table: string, where?: IWhere) {
    this.table = table;
    this.where = where;
  }

  accept = (visitor: IStatementVisitor): string => visitor.visit(this);
}
