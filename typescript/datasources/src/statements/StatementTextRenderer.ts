import type { ICondition } from "./ICondition.ts";
import type { IConditionVisitor } from "./IConditionVisitor.ts";
import type { IStatement } from "./IStatement.ts";
import type { IStatementVisitor } from "./IStatementVisitor.ts";
import { ConditionRenderer } from "./ConditionRenderer.ts";
import { CreateIndexStatement } from "./CreateIndexStatement.ts";
import { CreateTableStatement } from "./CreateTableStatement.ts";
import { DeleteStatement } from "./DeleteStatement.ts";
import { ExecuteStoredProcedureStatement } from "./ExecuteStoredProcedureStatement.ts";
import { ExpressionRenderer } from "./ExpressionRenderer.ts";
import { InsertStatement } from "./InsertStatement.ts";
import { ReturnStatement } from "./ReturnStatement.ts";
import { SelectInsertedIdStatement } from "./SelectInsertedIdStatement.ts";
import { SelectStatement } from "./SelectStatement.ts";
import { StatementRenderer } from "./StatementRenderer.ts";
import { UpdateStatement } from "./UpdateStatement.ts";
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
    if (target instanceof Where || target instanceof WhereText) return target.accept(this.conditions);
    if (
      target instanceof SelectStatement ||
      target instanceof InsertStatement ||
      target instanceof UpdateStatement ||
      target instanceof DeleteStatement ||
      target instanceof CreateTableStatement ||
      target instanceof CreateIndexStatement ||
      target instanceof ExecuteStoredProcedureStatement ||
      target instanceof ReturnStatement ||
      target instanceof SelectInsertedIdStatement
    )
      return target.accept(this.statements);
    return "where" in target || "fields" in target || "joins" in target || "column" in target
      ? (target as IStatement).accept(this.statements)
      : (target as ICondition).accept(this.conditions);
  }
}
