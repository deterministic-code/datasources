import type { ICreateRoutineStatement } from "./ICreateRoutineStatement.ts";
import type { IStatementVisitor } from "./IStatementVisitor.ts";

export type RoutineKind = "create" | "findOne" | "findAll" | "findBy" | "update" | "delete";

export type RoutineParameter = {
  name: string;
  type: string;
};

export class CreateRoutineStatement implements ICreateRoutineStatement {
  constructor(
    readonly name: string,
    readonly kind: RoutineKind,
    readonly table: string,
    readonly columns: readonly string[],
    readonly parameters: readonly RoutineParameter[],
    readonly key: string,
    readonly keyType: string,
    readonly writable: readonly string[],
    readonly byField?: string,
    readonly generatedKey = false,
    readonly auditUpdated?: string,
  ) {}

  accept = (visitor: IStatementVisitor): string => visitor.visit(this);
}
