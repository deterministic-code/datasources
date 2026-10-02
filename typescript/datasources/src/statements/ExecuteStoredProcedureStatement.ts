import type { Parameter } from "../Parameter.ts";
import type { IExecuteStoredProcedureStatement } from "./IExecuteStoredProcedureStatement.ts";
import type { IStatementVisitor } from "./IStatementVisitor.ts";

export class ExecuteStoredProcedureStatement implements IExecuteStoredProcedureStatement {
  readonly name: string;
  readonly parameters: Parameter[];

  constructor(name: string, parameters: Parameter[]) {
    this.name = name;
    this.parameters = parameters;
  }

  accept = (visitor: IStatementVisitor): string => visitor.visit(this);
}
