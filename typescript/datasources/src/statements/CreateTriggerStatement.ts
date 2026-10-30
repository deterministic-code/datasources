import type { ICreateTriggerStatement } from "./ICreateTriggerStatement.ts";
import type { IStatementVisitor } from "./IStatementVisitor.ts";

export class CreateTriggerStatement implements ICreateTriggerStatement {
  constructor(
    readonly table: string,
    readonly name: string,
    readonly updatedColumn: string,
    readonly keyColumn: string,
  ) {}

  accept = (visitor: IStatementVisitor): string => visitor.visit(this);
}
