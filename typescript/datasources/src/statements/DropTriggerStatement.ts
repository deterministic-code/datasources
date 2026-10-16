import type { IDropTriggerStatement } from "./IDropTriggerStatement.ts";
import type { IStatementVisitor } from "./IStatementVisitor.ts";

export class DropTriggerStatement implements IDropTriggerStatement {
  constructor(
    readonly name: string,
    readonly table: string,
  ) {}

  accept = (visitor: IStatementVisitor): string => visitor.visit(this);
}
