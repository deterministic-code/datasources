import type { IStatement } from "./IStatement.ts";
import type { IStatementVisitor } from "./IStatementVisitor.ts";

export class SelectInsertedIdStatement implements IStatement {
  constructor(
    readonly table: string,
    readonly column: string,
  ) {}

  accept = (visitor: IStatementVisitor): string => visitor.visit(this);
}
