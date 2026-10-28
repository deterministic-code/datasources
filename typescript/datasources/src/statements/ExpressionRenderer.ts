import type { Cell } from "../converters/Cell.ts";
import { ArrayExpression } from "./ArrayExpression.ts";
import { is } from "./brand.ts";
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
    if (is(expression, Where)) return this.render(expression.condition);
    if (is(expression, Identifier)) return expression.name;
    if (is(expression, Literal)) return inline(expression.value);
    if (is(expression, ArrayExpression)) return expression.elements.map((e) => this.render(e)).join(", ");
    if (is(expression, Operator.Not)) return `NOT (${this.render(expression.argument)})`;
    if (is(expression, Operator.IsNull)) return `${this.render(expression.argument)} IS NULL`;
    if (is(expression, Operator.IsNotNull)) return `${this.render(expression.argument)} IS NOT NULL`;
    if (is(expression, Operator.Exists)) return `EXISTS ${this.render(expression.argument)}`;
    if (is(expression, Subquery)) return `(${this.text.render(expression.query)})`;
    if (is(expression, Operator.In) && is(expression.left, Identifier) && is(expression.right, ArrayExpression))
      return expression.right.elements.length === 0
        ? "1 = 0"
        : `${expression.left.name} IN (${slots(expression.left.name, expression.right.elements.length)})`;
    if (is(expression, Operator.In) && is(expression.left, Identifier) && is(expression.right, Subquery))
      return `${expression.left.name} IN ${this.render(expression.right)}`;
    if (is(expression, Operator.NotIn) && is(expression.left, Identifier) && is(expression.right, ArrayExpression))
      return expression.right.elements.length === 0
        ? "1 = 1"
        : `${expression.left.name} NOT IN (${slots(expression.left.name, expression.right.elements.length)})`;
    if (is(expression, Operator.NotIn) && is(expression.left, Identifier) && is(expression.right, Subquery))
      return `${expression.left.name} NOT IN ${this.render(expression.right)}`;
    if (
      (is(expression, Operator.Between) || is(expression, Operator.NotBetween)) &&
      is(expression.value, Identifier) &&
      is(expression.low, Literal) &&
      is(expression.high, Literal)
    ) {
      const word = is(expression, Operator.NotBetween) ? "NOT BETWEEN" : "BETWEEN";
      return `${expression.value.name} ${word} $${parameterName(expression.value.name)}_0 AND $${parameterName(expression.value.name)}_1`;
    }
    if (is(expression, Operator.Between))
      return `${this.render(expression.value)} BETWEEN ${this.render(expression.low)} AND ${this.render(expression.high)}`;
    if (is(expression, Operator.NotBetween))
      return `${this.render(expression.value)} NOT BETWEEN ${this.render(expression.low)} AND ${this.render(expression.high)}`;
    if (is(expression, Operator.And))
      return `${this.wrap(expression.left, true)} AND ${this.wrap(expression.right, true)}`;
    if (is(expression, Operator.Or))
      return `${this.wrap(expression.left, false)} OR ${this.wrap(expression.right, false)}`;
    const operands = pair(expression as Operator);
    if (operands && is(operands.left, Identifier) && is(operands.right, Literal))
      return `${operands.left.name} ${symbol(expression as Operator)} $${parameterName(operands.left.name)}`;
    if (operands && is(operands.left, Literal) && is(operands.right, Literal))
      return `${inline(operands.left.value)} ${symbol(expression as Operator)} ${inline(operands.right.value)}`;
    if (operands) return `${this.render(operands.left)} ${symbol(expression as Operator)} ${this.render(operands.right)}`;
    throw new Error("empty expression");
  };

  private wrap = (child: IExpression, and: boolean): string =>
    needsParens(child, and) ? `(${this.render(child)})` : this.render(child);
}

const symbol = (op: Operator): string => {
  if (is(op, Operator.Equals)) return "=";
  if (is(op, Operator.NotEquals)) return "<>";
  if (is(op, Operator.GreaterThan)) return ">";
  if (is(op, Operator.LessThan)) return "<";
  if (is(op, Operator.GreaterThanOrEqual)) return ">=";
  if (is(op, Operator.LessThanOrEqual)) return "<=";
  if (is(op, Operator.Like)) return "LIKE";
  if (is(op, Operator.NotLike)) return "NOT LIKE";
  if (is(op, Operator.In)) return "IN";
  throw new Error("empty expression");
};

const inline = (value: Cell): string =>
  value === null
    ? "null"
    : typeof value === "boolean"
      ? String(value)
      : typeof value === "string"
        ? `'${value.replace(/'/g, "''")}'`
        : String(value);

const needsParens = (child: IExpression, and: boolean): boolean =>
  is(child, Where)
    ? needsParens(child.condition, and)
    : is(child, Operator.Not) ||
      is(child, Operator.In) ||
      is(child, Operator.NotIn) ||
      (is(child, Operator.And) && !and) ||
      (is(child, Operator.Or) && and);

const pair = (expression: Operator): { left: IExpression; right: IExpression } | undefined =>
  is(expression, Operator.Equals) ||
  is(expression, Operator.NotEquals) ||
  is(expression, Operator.GreaterThan) ||
  is(expression, Operator.LessThan) ||
  is(expression, Operator.GreaterThanOrEqual) ||
  is(expression, Operator.LessThanOrEqual) ||
  is(expression, Operator.And) ||
  is(expression, Operator.Or) ||
  is(expression, Operator.In) ||
  is(expression, Operator.Like) ||
  is(expression, Operator.NotLike) ||
  is(expression, Operator.NotIn)
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
