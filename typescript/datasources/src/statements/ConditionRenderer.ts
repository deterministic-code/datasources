import type { ICondition } from "./ICondition.ts";
import type { IConditionVisitor } from "./IConditionVisitor.ts";
import type { ExpressionRenderer } from "./ExpressionRenderer.ts";
import { is } from "./brand.ts";
import { Where } from "./Where.ts";
import { WhereText } from "./WhereText.ts";

export class ConditionRenderer implements IConditionVisitor {
  constructor(private readonly expressions: ExpressionRenderer) {}

  render = (condition: ICondition): string => condition.accept(this);

  visit(where: Where): string;
  visit(where: WhereText): string;
  visit(where: Where | WhereText): string {
    return is(where, Where)
      ? this.expressions.render(where.condition)
      : is(where, WhereText)
        ? where.rawText
        : (() => {
            throw new Error("empty expression");
          })();
  }
}
