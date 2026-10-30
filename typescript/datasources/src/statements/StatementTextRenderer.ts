import type { ICondition } from "./ICondition.ts";
import type { IConditionVisitor } from "./IConditionVisitor.ts";
import type { IStatement } from "./IStatement.ts";
import type { IStatementVisitor } from "./IStatementVisitor.ts";
import { is } from "./brand.ts";
import { ConditionRenderer } from "./ConditionRenderer.ts";
import { ExpressionRenderer } from "./ExpressionRenderer.ts";
import { KeywordCasing } from "./KeywordCasing.ts";
import type { ProcedureSource } from "./ProcedureSource.ts";
import { StatementRenderer } from "./StatementRenderer.ts";
import { StatementNaming } from "./StatementNaming.ts";
import { Where } from "./Where.ts";
import { WhereText } from "./WhereText.ts";

export class StatementTextRenderer {
  protected readonly statements: IStatementVisitor;
  protected readonly conditions: IConditionVisitor;
  readonly naming: StatementNaming;
  private readonly statementsFactory?: (text: StatementTextRenderer) => StatementRenderer;
  private readonly conditionsFactory?: (expressions: ExpressionRenderer) => ConditionRenderer;
  private readonly expressionsFactory?: (text: StatementTextRenderer) => ExpressionRenderer;
  private readonly applyKeywordsFn?: (sql: string, keyword: (word: string) => string) => string;

  constructor(
    statements?: (text: StatementTextRenderer) => StatementRenderer,
    conditions?: (expressions: ExpressionRenderer) => ConditionRenderer,
    expressions?: (text: StatementTextRenderer) => ExpressionRenderer,
    naming?: StatementNaming,
    applyKeywords?: (sql: string, keyword: (word: string) => string) => string,
  ) {
    this.statementsFactory = statements;
    this.conditionsFactory = conditions;
    this.expressionsFactory = expressions;
    this.applyKeywordsFn = applyKeywords;
    this.naming = naming ?? StatementNaming.sql;
    const expressionRenderer = expressions?.(this) ?? new ExpressionRenderer(this);
    this.statements = statements?.(this) ?? new StatementRenderer(this);
    this.conditions = conditions?.(expressionRenderer) ?? new ConditionRenderer(expressionRenderer);
  }

  withNaming = (naming: StatementNaming): StatementTextRenderer =>
    new StatementTextRenderer(
      this.statementsFactory,
      this.conditionsFactory,
      this.expressionsFactory,
      naming,
      this.applyKeywordsFn,
    );

  procedures = (source: ProcedureSource): IStatement[] =>
    this.statements instanceof StatementRenderer ? this.statements.procedures(source) : [];

  applyKeywords = (sql: string): string =>
    this.naming.keywordCasing === KeywordCasing.Lower && this.applyKeywordsFn !== undefined
      ? this.applyKeywordsFn(sql, this.naming.keyword)
      : sql;

  render(statement: IStatement): string;
  render(condition: ICondition): string;
  render(target: IStatement | ICondition): string {
    return is(target, Where) || is(target, WhereText)
      ? target.accept(this.conditions)
      : (target as IStatement).accept(this.statements);
  }
}
