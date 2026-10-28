import type { IExpression } from "./IExpression.ts";
import type { SelectStatement } from "./SelectStatement.ts";
import { brand } from "./brand.ts";

export class Subquery implements IExpression {
  constructor(readonly query: SelectStatement) {}
}

brand(Subquery);
