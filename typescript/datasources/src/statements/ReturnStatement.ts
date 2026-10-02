import type { Parameter } from "../Parameter.ts";
import type { IReturnStatement } from "./IReturnStatement.ts";
import type { IStatementVisitor } from "./IStatementVisitor.ts";

export class ReturnStatement implements IReturnStatement {
  readonly name: string;
  readonly parameters: Parameter[];

  constructor(name: string, parameters: Parameter[]) {
    this.name = name;
    this.parameters = parameters;
  }

  accept = (visitor: IStatementVisitor): string => visitor.visit(this);
}
