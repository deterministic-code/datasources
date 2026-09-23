import type { Parameter } from "../Parameter.ts";
import type { Scalar } from "../builders/Scalar.ts";
import { ArrayExpression } from "./ArrayExpression.ts";
import { Identifier } from "./Identifier.ts";
import type { IExpression } from "./IExpression.ts";
import type { IWhere } from "./IWhere.ts";
import { Literal } from "./Literal.ts";
import { Operator } from "./Operator.ts";
import type { Comparison } from "./Cmp.ts";
import { eq } from "./Cmp.ts";

const symbol = (op: Operator): string =>
  op instanceof Operator.Equals
    ? "="
    : op instanceof Operator.NotEquals
      ? "<>"
      : op instanceof Operator.GreaterThan
        ? ">"
        : op instanceof Operator.LessThan
          ? "<"
          : op instanceof Operator.GreaterThanOrEqual
            ? ">="
            : op instanceof Operator.LessThanOrEqual
              ? "<="
              : "IN";

const inline = (value: Scalar): string =>
  value === null ? "null" : typeof value === "boolean" ? String(value) : String(value);

const needsParens = (child: IExpression, and: boolean): boolean =>
  child instanceof WhereClause
    ? needsParens(child.condition, and)
    : child instanceof Operator.Not ||
      child instanceof Operator.In ||
      (child instanceof Operator.And && !and) ||
      (child instanceof Operator.Or && and);

const wrap = (child: IExpression, and: boolean): string =>
  needsParens(child, and) ? `(${render(child)})` : render(child);

const operands = (
  expression: Operator,
): { left: IExpression; right: IExpression } | undefined =>
  expression instanceof Operator.Equals ||
  expression instanceof Operator.NotEquals ||
  expression instanceof Operator.GreaterThan ||
  expression instanceof Operator.LessThan ||
  expression instanceof Operator.GreaterThanOrEqual ||
  expression instanceof Operator.LessThanOrEqual ||
  expression instanceof Operator.And ||
  expression instanceof Operator.Or ||
  expression instanceof Operator.In
    ? expression
    : undefined;

const render = (expression: IExpression): string =>
  expression instanceof WhereClause
    ? render(expression.condition)
    : expression instanceof Identifier
      ? expression.name
      : expression instanceof Literal
        ? inline(expression.value)
        : expression instanceof ArrayExpression
          ? expression.elements.map(render).join(", ")
          : expression instanceof Operator.Not
            ? `NOT (${render(expression.argument)})`
            : expression instanceof Operator.In &&
                expression.left instanceof Identifier &&
                expression.right instanceof ArrayExpression
              ? expression.right.elements.length === 0
                ? "1 = 0"
                : `${expression.left.name} IN (${expression.right.elements.map((_, i) => `$${expression.left.name}_${i}`).join(", ")})`
              : (() => {
                  const pair = expression instanceof Operator ? operands(expression) : undefined;
                  return pair && pair.left instanceof Identifier && pair.right instanceof Literal
                    ? `${pair.left.name} ${symbol(expression as Operator)} $${pair.left.name}`
                    : pair && pair.left instanceof Literal && pair.right instanceof Literal
                      ? `${inline(pair.left.value)} ${symbol(expression as Operator)} ${inline(pair.right.value)}`
                      : expression instanceof Operator.And
                        ? `${wrap(expression.left, true)} AND ${wrap(expression.right, true)}`
                        : expression instanceof Operator.Or
                          ? `${wrap(expression.left, false)} OR ${wrap(expression.right, false)}`
                          : pair
                            ? `${render(pair.left)} ${symbol(expression as Operator)} ${render(pair.right)}`
                            : (() => {
                                throw new Error("empty expression");
                              })();
                })();

const collect = (expression: IExpression): Parameter<Scalar>[] =>
  expression instanceof WhereClause
    ? collect(expression.condition)
    : expression instanceof Literal || expression instanceof Identifier
      ? []
      : expression instanceof Operator.Not
        ? collect(expression.argument)
        : expression instanceof Operator.In &&
            expression.left instanceof Identifier &&
            expression.right instanceof ArrayExpression
          ? expression.right.elements.map((element, i) =>
              element instanceof Literal
                ? { name: `${expression.left.name}_${i}`, value: element.value }
                : (() => {
                    throw new Error("empty in");
                  })(),
            )
          : (() => {
              const pair = expression instanceof Operator ? operands(expression) : undefined;
              return pair && pair.left instanceof Identifier && pair.right instanceof Literal
                ? [{ name: pair.left.name, value: pair.right.value }]
                : pair && pair.left instanceof Literal && pair.right instanceof Literal
                  ? []
                  : pair
                    ? [...collect(pair.left), ...collect(pair.right)]
                    : expression instanceof ArrayExpression
                      ? expression.elements.flatMap(collect)
                      : (() => {
                          throw new Error("empty expression");
                        })();
            })();

export class WhereClause implements IWhere {
  readonly condition: IExpression;

  constructor(condition: IExpression);
  constructor(column: string, op: Comparison, value: Scalar);
  constructor(columnOrCondition: IExpression | string, op: Comparison = eq, value: Scalar = null) {
    this.condition =
      typeof columnOrCondition === "string"
        ? op(columnOrCondition, value)
        : columnOrCondition;
  }

  and(column: string, op: Comparison, value: Scalar): WhereClause {
    return new WhereClause(new Operator.And(this.condition, op(column, value)));
  }

  or(column: string, op: Comparison, value: Scalar): WhereClause {
    return new WhereClause(new Operator.Or(this.condition, op(column, value)));
  }

  getText = (): string => render(this.condition);
  get parameters(): Parameter<Scalar>[] {
    return collect(this.condition);
  }
}
