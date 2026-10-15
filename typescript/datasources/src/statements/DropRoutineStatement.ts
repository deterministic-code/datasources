import type { IDropRoutineStatement } from "./IDropRoutineStatement.ts";
import type { IStatementVisitor } from "./IStatementVisitor.ts";

export class DropRoutineStatement implements IDropRoutineStatement {
  constructor(readonly name: string) {}

  accept = (visitor: IStatementVisitor): string => visitor.visit(this);
}
