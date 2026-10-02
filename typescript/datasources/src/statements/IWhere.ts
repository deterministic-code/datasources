import type { Parameter } from "../Parameter.ts";
import type { ICondition } from "./ICondition.ts";
import type { IExpression } from "./IExpression.ts";

export interface IWhere extends IExpression, ICondition {
  readonly parameters?: Parameter[];
}
