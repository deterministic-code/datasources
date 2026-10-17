import type { ICreateBulkCreateStoredProcedureStatement } from "./ICreateBulkCreateStoredProcedureStatement.ts";
import type { IStatementVisitor } from "./IStatementVisitor.ts";
import type { RoutineParameter } from "./RoutineParameter.ts";

export class CreateBulkCreateStoredProcedureStatement implements ICreateBulkCreateStoredProcedureStatement {
  constructor(
    readonly name: string,
    readonly table: string,
    readonly columns: readonly RoutineParameter[],
    readonly payload: RoutineParameter,
    readonly key: string,
    readonly keyType: string,
  ) {}

  accept = (visitor: IStatementVisitor): string => visitor.visit(this);
}
