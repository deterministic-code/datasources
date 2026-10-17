import type { ICreateBulkDeleteStoredProcedureStatement } from "./ICreateBulkDeleteStoredProcedureStatement.ts";
import type { IStatementVisitor } from "./IStatementVisitor.ts";
import type { RoutineParameter } from "./RoutineParameter.ts";

export class CreateBulkDeleteStoredProcedureStatement implements ICreateBulkDeleteStoredProcedureStatement {
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
