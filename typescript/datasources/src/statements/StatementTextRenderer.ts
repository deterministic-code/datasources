import type { ICondition } from "./ICondition.ts";
import type { IConditionVisitor } from "./IConditionVisitor.ts";
import type { IStatement } from "./IStatement.ts";
import type { IStatementVisitor } from "./IStatementVisitor.ts";
import { ConditionRenderer } from "./ConditionRenderer.ts";
import { ExpressionRenderer } from "./ExpressionRenderer.ts";
import { StatementRenderer } from "./StatementRenderer.ts";
import { Where } from "./Where.ts";
import { WhereText } from "./WhereText.ts";

export class StatementTextRenderer {
  protected readonly statements: IStatementVisitor;
  protected readonly conditions: IConditionVisitor;

  constructor(
    statements?: (text: StatementTextRenderer) => StatementRenderer,
    conditions?: (expressions: ExpressionRenderer) => ConditionRenderer,
    expressions?: (text: StatementTextRenderer) => ExpressionRenderer,
  ) {
    const expressionRenderer = expressions?.(this) ?? new ExpressionRenderer(this);
    this.statements = statements?.(this) ?? new StatementRenderer(this);
    this.conditions = conditions?.(expressionRenderer) ?? new ConditionRenderer(expressionRenderer);
  }

  render(statement: IStatement): string;
  render(condition: ICondition): string;
  render(target: IStatement | ICondition): string {
    if (target.constructor.name === "Where" || target.constructor.name === "WhereText") {
      return (target as ICondition).accept(this.conditions);
    }
    if (target.constructor.name.endsWith("Statement")) {
      return (target as IStatement).accept(this.statements);
    }
    if (target instanceof Where || target instanceof WhereText) return target.accept(this.conditions);
    return "where" in target || "fields" in target || "joins" in target || "column" in target
      ? (target as IStatement).accept(this.statements)
      : (target as ICondition).accept(this.conditions);
  }
}
