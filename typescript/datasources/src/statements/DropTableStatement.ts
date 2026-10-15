import type { IDropTableStatement } from "./IDropTableStatement.ts";
import type { IStatementVisitor } from "./IStatementVisitor.ts";

export class DropTableStatement implements IDropTableStatement {
  constructor(readonly table: string) {}

  accept = (visitor: IStatementVisitor): string => visitor.visit(this);
}
