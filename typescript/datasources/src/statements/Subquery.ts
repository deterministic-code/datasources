import type { IExpression } from "./IExpression.ts";
import type { SelectStatement } from "./SelectStatement.ts";

export class Subquery implements IExpression {
  constructor(readonly query: SelectStatement) {}
}
