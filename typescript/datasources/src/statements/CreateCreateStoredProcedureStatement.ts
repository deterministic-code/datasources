import type { ICreateCreateStoredProcedureStatement } from "./ICreateCreateStoredProcedureStatement.ts";
import type { IStatementVisitor } from "./IStatementVisitor.ts";
import type { RoutineParameter } from "./RoutineParameter.ts";

export class CreateCreateStoredProcedureStatement implements ICreateCreateStoredProcedureStatement {
  constructor(
    readonly name: string,
    readonly table: string,
    readonly columns: readonly string[],
    readonly parameters: readonly RoutineParameter[],
    readonly key: string,
    readonly keyType: string,
    readonly writable: readonly string[],
    readonly generatedKey = false,
    readonly auditUpdated?: string,
  ) {}

  accept = (visitor: IStatementVisitor): string => visitor.visit(this);
}
