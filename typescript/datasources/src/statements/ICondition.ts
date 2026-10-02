import type { IConditionVisitor } from "./IConditionVisitor.ts";

export interface ICondition {
  accept(visitor: IConditionVisitor): string;
}
