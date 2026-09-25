import type { IExpression } from "./IExpression.ts";
import type { QueryStatement } from "./QueryStatement.ts";

export class Subquery implements IExpression {
  constructor(readonly query: QueryStatement) {}
}
