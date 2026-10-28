import type { ICondition } from "./ICondition.ts";
import type { IConditionVisitor } from "./IConditionVisitor.ts";
import type { ExpressionRenderer } from "./ExpressionRenderer.ts";
import { Where } from "./Where.ts";
import { WhereText } from "./WhereText.ts";

export class ConditionRenderer implements IConditionVisitor {
  constructor(private readonly expressions: ExpressionRenderer) {}

  render = (condition: ICondition): string => condition.accept(this);

  visit(where: Where): string;
  visit(where: WhereText): string;
  visit(where: Where | WhereText): string {
    return where instanceof Where || where.constructor.name === "Where"
      ? this.expressions.render((where as Where).condition)
      : where.rawText;
  }
}
