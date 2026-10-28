import type { Cell } from "../converters/Cell.ts";
import { ArrayExpression } from "./ArrayExpression.ts";
import type { IExpression } from "./IExpression.ts";
import { Identifier } from "./Identifier.ts";
import { Literal } from "./Literal.ts";
import { Operator } from "./Operator.ts";
import type { StatementTextRenderer } from "./StatementTextRenderer.ts";
import { Subquery } from "./Subquery.ts";
import { Where } from "./Where.ts";

const is = <T extends object>(value: object, type: abstract new (...args: never[]) => T, name: string): value is T =>
  value instanceof type || value.constructor.name === name;

export class ExpressionRenderer {
  constructor(private readonly text: StatementTextRenderer) {}

  render = (expression: IExpression): string => {
    if (is(expression, Where, "Where")) return this.render(expression.condition);
    if (is(expression, Identifier, "Identifier")) return expression.name;
    if (is(expression, Literal, "Literal")) return inline(expression.value);
    if (is(expression, ArrayExpression, "ArrayExpression")) return expression.elements.map((e) => this.render(e)).join(", ");
    if (is(expression, Operator.Not, "Not")) return `NOT (${this.render(expression.argument)})`;
    if (is(expression, Operator.IsNull, "IsNull")) return `${this.render(expression.argument)} IS NULL`;
    if (is(expression, Operator.IsNotNull, "IsNotNull")) return `${this.render(expression.argument)} IS NOT NULL`;
    if (is(expression, Operator.Exists, "Exists")) return `EXISTS ${this.render(expression.argument)}`;
    if (is(expression, Subquery, "Subquery")) return `(${this.text.render(expression.query)})`;
    if (
      is(expression, Operator.In, "In") &&
      is(expression.left, Identifier, "Identifier") &&
      is(expression.right, ArrayExpression, "ArrayExpression")
    )
      return expression.right.elements.length === 0
        ? "1 = 0"
        : `${expression.left.name} IN (${slots(expression.left.name, expression.right.elements.length)})`;
    if (
      is(expression, Operator.In, "In") &&
      is(expression.left, Identifier, "Identifier") &&
      is(expression.right, Subquery, "Subquery")
    )
      return `${expression.left.name} IN ${this.render(expression.right)}`;
    if (
      is(expression, Operator.NotIn, "NotIn") &&
      is(expression.left, Identifier, "Identifier") &&
      is(expression.right, ArrayExpression, "ArrayExpression")
    )
      return expression.right.elements.length === 0
        ? "1 = 1"
        : `${expression.left.name} NOT IN (${slots(expression.left.name, expression.right.elements.length)})`;
    if (
      is(expression, Operator.NotIn, "NotIn") &&
      is(expression.left, Identifier, "Identifier") &&
      is(expression.right, Subquery, "Subquery")
    )
      return `${expression.left.name} NOT IN ${this.render(expression.right)}`;
    if (
      (is(expression, Operator.Between, "Between") || is(expression, Operator.NotBetween, "NotBetween")) &&
      is(expression.value, Identifier, "Identifier") &&
      is(expression.low, Literal, "Literal") &&
      is(expression.high, Literal, "Literal")
    ) {
      const word = is(expression, Operator.NotBetween, "NotBetween") ? "NOT BETWEEN" : "BETWEEN";
      return `${expression.value.name} ${word} $${parameterName(expression.value.name)}_0 AND $${parameterName(expression.value.name)}_1`;
    }
    if (is(expression, Operator.Between, "Between"))
      return `${this.render(expression.value)} BETWEEN ${this.render(expression.low)} AND ${this.render(expression.high)}`;
    if (is(expression, Operator.NotBetween, "NotBetween"))
      return `${this.render(expression.value)} NOT BETWEEN ${this.render(expression.low)} AND ${this.render(expression.high)}`;
    if (is(expression, Operator.And, "And"))
      return `${this.wrap(expression.left, true)} AND ${this.wrap(expression.right, true)}`;
    if (is(expression, Operator.Or, "Or"))
      return `${this.wrap(expression.left, false)} OR ${this.wrap(expression.right, false)}`;
    const operands = pair(expression as Operator);
    if (operands && is(operands.left, Identifier, "Identifier") && is(operands.right, Literal, "Literal"))
      return `${operands.left.name} ${symbol(expression as Operator)} $${parameterName(operands.left.name)}`;
    if (operands && is(operands.left, Literal, "Literal") && is(operands.right, Literal, "Literal"))
      return `${inline(operands.left.value)} ${symbol(expression as Operator)} ${inline(operands.right.value)}`;
    if (operands)
      return `${this.render(operands.left)} ${symbol(expression as Operator)} ${this.render(operands.right)}`;
    throw new Error("empty expression");
  };

  private wrap = (child: IExpression, and: boolean): string =>
    needsParens(child, and) ? `(${this.render(child)})` : this.render(child);
}

const symbols: Record<string, string> = {
  Equals: "=",
  NotEquals: "<>",
  GreaterThan: ">",
  LessThan: "<",
  GreaterThanOrEqual: ">=",
  LessThanOrEqual: "<=",
  Like: "LIKE",
  NotLike: "NOT LIKE",
  In: "IN",
};

const symbol = (op: Operator): string => symbols[op.constructor.name] as string;

const inline = (value: Cell): string =>
  value === null
    ? "null"
    : typeof value === "boolean"
      ? String(value)
      : typeof value === "string"
        ? `'${value.replace(/'/g, "''")}'`
        : String(value);

const needsParens = (child: IExpression, and: boolean): boolean =>
  is(child, Where, "Where")
    ? needsParens(child.condition, and)
    : is(child, Operator.Not, "Not") ||
      is(child, Operator.In, "In") ||
      is(child, Operator.NotIn, "NotIn") ||
      (is(child, Operator.And, "And") && !and) ||
      (is(child, Operator.Or, "Or") && and);

const pair = (
  expression: Operator,
): { left: IExpression; right: IExpression } | undefined =>
  is(expression, Operator.Equals, "Equals") ||
  is(expression, Operator.NotEquals, "NotEquals") ||
  is(expression, Operator.GreaterThan, "GreaterThan") ||
  is(expression, Operator.LessThan, "LessThan") ||
  is(expression, Operator.GreaterThanOrEqual, "GreaterThanOrEqual") ||
  is(expression, Operator.LessThanOrEqual, "LessThanOrEqual") ||
  is(expression, Operator.And, "And") ||
  is(expression, Operator.Or, "Or") ||
  is(expression, Operator.In, "In") ||
  is(expression, Operator.Like, "Like") ||
  is(expression, Operator.NotLike, "NotLike")
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
