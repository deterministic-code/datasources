import type { Cell } from "../converters/Cell.ts";
import { ArrayExpression } from "./ArrayExpression.ts";
import type { IExpression } from "./IExpression.ts";
import { Identifier } from "./Identifier.ts";
import { Literal } from "./Literal.ts";
import { Operator } from "./Operator.ts";
import type { StatementTextRenderer } from "./StatementTextRenderer.ts";
import { Subquery } from "./Subquery.ts";
import { Where } from "./Where.ts";

export class ExpressionRenderer {
  constructor(private readonly text: StatementTextRenderer) {}

  render = (expression: IExpression): string => {
    if (expression instanceof Where) return this.render(expression.condition);
    if (expression instanceof Identifier) return expression.name;
    if (expression instanceof Literal) return inline(expression.value);
    if (expression instanceof ArrayExpression) return expression.elements.map((e) => this.render(e)).join(", ");
    if (expression instanceof Operator.Not) return `NOT (${this.render(expression.argument)})`;
    if (expression instanceof Operator.IsNull) return `${this.render(expression.argument)} IS NULL`;
    if (expression instanceof Operator.IsNotNull) return `${this.render(expression.argument)} IS NOT NULL`;
    if (expression instanceof Operator.Exists) return `EXISTS ${this.render(expression.argument)}`;
    if (expression instanceof Subquery) return `(${this.text.render(expression.query)})`;
    if (
      expression instanceof Operator.In &&
      expression.left instanceof Identifier &&
      expression.right instanceof ArrayExpression
    )
      return expression.right.elements.length === 0
        ? "1 = 0"
        : `${expression.left.name} IN (${slots(expression.left.name, expression.right.elements.length)})`;
    if (
      expression instanceof Operator.In &&
      expression.left instanceof Identifier &&
      expression.right instanceof Subquery
    )
      return `${expression.left.name} IN ${this.render(expression.right)}`;
    if (
      expression instanceof Operator.NotIn &&
      expression.left instanceof Identifier &&
      expression.right instanceof ArrayExpression
    )
      return expression.right.elements.length === 0
        ? "1 = 1"
        : `${expression.left.name} NOT IN (${slots(expression.left.name, expression.right.elements.length)})`;
    if (
      expression instanceof Operator.NotIn &&
      expression.left instanceof Identifier &&
      expression.right instanceof Subquery
    )
      return `${expression.left.name} NOT IN ${this.render(expression.right)}`;
    if (
      (expression instanceof Operator.Between || expression instanceof Operator.NotBetween) &&
      expression.value instanceof Identifier &&
      expression.low instanceof Literal &&
      expression.high instanceof Literal
    ) {
      const word = expression instanceof Operator.NotBetween ? "NOT BETWEEN" : "BETWEEN";
      return `${expression.value.name} ${word} $${parameterName(expression.value.name)}_0 AND $${parameterName(expression.value.name)}_1`;
    }
    if (expression instanceof Operator.Between)
      return `${this.render(expression.value)} BETWEEN ${this.render(expression.low)} AND ${this.render(expression.high)}`;
    if (expression instanceof Operator.NotBetween)
      return `${this.render(expression.value)} NOT BETWEEN ${this.render(expression.low)} AND ${this.render(expression.high)}`;
    if (expression instanceof Operator.And)
      return `${this.wrap(expression.left, true)} AND ${this.wrap(expression.right, true)}`;
    if (expression instanceof Operator.Or)
      return `${this.wrap(expression.left, false)} OR ${this.wrap(expression.right, false)}`;
    const operands = expression instanceof Operator ? pair(expression) : undefined;
    if (operands && operands.left instanceof Identifier && operands.right instanceof Literal)
      return `${operands.left.name} ${symbol(expression as Operator)} $${parameterName(operands.left.name)}`;
    if (operands && operands.left instanceof Literal && operands.right instanceof Literal)
      return `${inline(operands.left.value)} ${symbol(expression as Operator)} ${inline(operands.right.value)}`;
    if (operands)
      return `${this.render(operands.left)} ${symbol(expression as Operator)} ${this.render(operands.right)}`;
    throw new Error("empty expression");
  };

  private wrap = (child: IExpression, and: boolean): string =>
    needsParens(child, and) ? `(${this.render(child)})` : this.render(child);
}

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
              : op instanceof Operator.Like
                ? "LIKE"
                : op instanceof Operator.NotLike
                  ? "NOT LIKE"
                  : "IN";

const inline = (value: Cell): string =>
  value === null
    ? "null"
    : typeof value === "boolean"
      ? String(value)
      : typeof value === "string"
        ? `'${value.replace(/'/g, "''")}'`
        : String(value);

const needsParens = (child: IExpression, and: boolean): boolean =>
  child instanceof Where
    ? needsParens(child.condition, and)
    : child instanceof Operator.Not ||
      child instanceof Operator.In ||
      child instanceof Operator.NotIn ||
      (child instanceof Operator.And && !and) ||
      (child instanceof Operator.Or && and);

const pair = (
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
  expression instanceof Operator.In ||
  expression instanceof Operator.Like ||
  expression instanceof Operator.NotLike
    ? expression
    : undefined;

const parameterName = (name: string): string =>
  parts(name)
    .map((part) =>
      part.length >= 2 && part.startsWith('"') && part.endsWith('"')
        ? part.slice(1, -1).replace(/""/g, '"')
        : part,
    )
    .join(".");

const parts = (name: string): string[] => {
  const found: string[] = [];
  let start = 0;
  let quoted = false;
  for (let i = 0; i < name.length; i++) {
    if (name[i] === '"') quoted = !quoted;
    if (name[i] !== "." || quoted) continue;
    found.push(name.slice(start, i));
    start = i + 1;
  }
  found.push(name.slice(start));
  return found;
};

const slots = (name: string, count: number): string =>
  Array.from({ length: count }, (_, i) => `$${parameterName(name)}_${i}`).join(", ");
