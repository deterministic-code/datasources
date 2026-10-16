import type { IDropIndexStatement } from "./IDropIndexStatement.ts";
import type { IStatementVisitor } from "./IStatementVisitor.ts";

export class DropIndexStatement implements IDropIndexStatement {
  constructor(
    readonly name: string,
    readonly table: string,
  ) {}

  accept = (visitor: IStatementVisitor): string => visitor.visit(this);
}
