import type { Parameter } from "../Parameter.ts";
import type { Cell } from "../converters/Cell.ts";
import { ArrayExpression } from "./ArrayExpression.ts";
import { Identifier } from "./Identifier.ts";
import type { IConditionVisitor } from "./IConditionVisitor.ts";
import type { IExpression } from "./IExpression.ts";
import type { IWhere } from "./IWhere.ts";
import { Literal } from "./Literal.ts";
import { Operator } from "./Operator.ts";
import type { Comparison } from "./Cmp.ts";
import { eq } from "./Cmp.ts";
import { SelectStatement } from "./SelectStatement.ts";
import { Subquery } from "./Subquery.ts";

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

const side = (
  expression: IExpression,
  other: IExpression,
  column: (name: string) => string,
  value: (name: string, cell: Cell) => Cell,
): IExpression =>
  expression instanceof Literal && other instanceof Identifier
    ? new Literal(value(other.name, expression.value))
    : rewrite(expression, column, value);

const inRight = (
  left: IExpression,
  right: IExpression,
  column: (name: string) => string,
  value: (name: string, cell: Cell) => Cell,
): IExpression =>
  left instanceof Identifier && right instanceof ArrayExpression
    ? new ArrayExpression(
        right.elements.map((element) =>
          element instanceof Literal
            ? new Literal(value(left.name, element.value))
            : rewrite(element, column, value),
        ),
      )
    : rewrite(right, column, value);

const bound = (
  subject: IExpression,
  edge: IExpression,
  column: (name: string) => string,
  value: (name: string, cell: Cell) => Cell,
): IExpression =>
  subject instanceof Identifier && edge instanceof Literal
    ? new Literal(value(subject.name, edge.value))
    : rewrite(edge, column, value);

const rewriteWhere = (
  where: IWhere | undefined,
  column: (name: string) => string,
  value: (name: string, cell: Cell) => Cell,
): IWhere | undefined =>
  where instanceof Where ? new Where(rewrite(where.condition, column, value)) : where;

const rewriteQuery = (
  query: SelectStatement,
  column: (name: string) => string,
  value: (name: string, cell: Cell) => Cell,
): SelectStatement =>
  query.table === undefined
    ? query
    : new SelectStatement({
        distinct: query.distinct,
        table: query.table,
        fields: query.fields,
        aliases: query.aliases,
        where: rewriteWhere(query.where, column, value),
        joins: query.joins.map((join) =>
          join.on ? { ...join, on: rewrite(join.on, column, value) } : join,
        ),
        groupBy: query.groupBy,
        having: rewriteWhere(query.having, column, value),
        orderBy: query.orderBy,
        limit: query.limit,
        offset: query.offset,
      });

const rewrite = (
  expression: IExpression,
  column: (name: string) => string,
  value: (name: string, cell: Cell) => Cell,
): IExpression =>
  expression instanceof Where
    ? rewrite(expression.condition, column, value)
    : expression instanceof Identifier
      ? new Identifier(column(expression.name))
      : expression instanceof Literal
        ? expression
        : expression instanceof ArrayExpression
          ? new ArrayExpression(expression.elements.map((element) => rewrite(element, column, value)))
          : expression instanceof Subquery
            ? new Subquery(rewriteQuery(expression.query, column, value))
            : expression instanceof Operator.Not
              ? new Operator.Not(rewrite(expression.argument, column, value))
              : expression instanceof Operator.IsNull
                ? new Operator.IsNull(rewrite(expression.argument, column, value))
                : expression instanceof Operator.IsNotNull
                  ? new Operator.IsNotNull(rewrite(expression.argument, column, value))
                  : expression instanceof Operator.Exists
                    ? new Operator.Exists(rewrite(expression.argument, column, value))
                    : expression instanceof Operator.And
                      ? new Operator.And(rewrite(expression.left, column, value), rewrite(expression.right, column, value))
                      : expression instanceof Operator.Or
                        ? new Operator.Or(rewrite(expression.left, column, value), rewrite(expression.right, column, value))
                        : expression instanceof Operator.In
                          ? new Operator.In(rewrite(expression.left, column, value), inRight(expression.left, expression.right, column, value))
                          : expression instanceof Operator.NotIn
                            ? new Operator.NotIn(rewrite(expression.left, column, value), inRight(expression.left, expression.right, column, value))
                            : expression instanceof Operator.Like
                              ? new Operator.Like(side(expression.left, expression.right, column, value), side(expression.right, expression.left, column, value))
                              : expression instanceof Operator.NotLike
                                ? new Operator.NotLike(side(expression.left, expression.right, column, value), side(expression.right, expression.left, column, value))
                                : expression instanceof Operator.Between
                                  ? new Operator.Between(rewrite(expression.value, column, value), bound(expression.value, expression.low, column, value), bound(expression.value, expression.high, column, value))
                                  : expression instanceof Operator.NotBetween
                                    ? new Operator.NotBetween(rewrite(expression.value, column, value), bound(expression.value, expression.low, column, value), bound(expression.value, expression.high, column, value))
                                    : expression instanceof Operator.Equals
                                      ? new Operator.Equals(side(expression.left, expression.right, column, value), side(expression.right, expression.left, column, value))
                                      : expression instanceof Operator.NotEquals
                                        ? new Operator.NotEquals(side(expression.left, expression.right, column, value), side(expression.right, expression.left, column, value))
                                        : expression instanceof Operator.GreaterThan
                                          ? new Operator.GreaterThan(side(expression.left, expression.right, column, value), side(expression.right, expression.left, column, value))
                                          : expression instanceof Operator.LessThan
                                            ? new Operator.LessThan(side(expression.left, expression.right, column, value), side(expression.right, expression.left, column, value))
                                            : expression instanceof Operator.GreaterThanOrEqual
                                              ? new Operator.GreaterThanOrEqual(side(expression.left, expression.right, column, value), side(expression.right, expression.left, column, value))
                                              : expression instanceof Operator.LessThanOrEqual
                                                ? new Operator.LessThanOrEqual(side(expression.left, expression.right, column, value), side(expression.right, expression.left, column, value))
                                                : (() => {
                                                    throw new Error("empty expression");
                                                  })();

const listed = (name: string, elements: IExpression[]): Parameter<Cell>[] =>
  elements.map((element, i) =>
    element instanceof Literal
      ? { name: `${parameterName(name)}_${i}`, value: element.value }
      : (() => {
          throw new Error("empty in");
        })(),
  );

const collect = (expression: IExpression): Parameter<Cell>[] => {
  if (expression instanceof Where) return collect(expression.condition);
  if (expression instanceof Literal || expression instanceof Identifier) return [];
  if (expression instanceof Operator.Not || expression instanceof Operator.IsNull || expression instanceof Operator.IsNotNull)
    return collect(expression.argument);
  if (expression instanceof Operator.Exists) return collect(expression.argument);
  if (expression instanceof Subquery)
    return [
      ...(expression.query.where ? collect(expression.query.where) : []),
      ...(expression.query.having ? collect(expression.query.having) : []),
      ...expression.query.joins.flatMap((join) => (join.on ? collect(join.on) : [])),
    ];
  if (expression instanceof Operator.In && expression.left instanceof Identifier && expression.right instanceof ArrayExpression)
    return listed(expression.left.name, expression.right.elements);
  if (expression instanceof Operator.NotIn && expression.left instanceof Identifier && expression.right instanceof ArrayExpression)
    return listed(expression.left.name, expression.right.elements);
  if (expression instanceof Operator.In || expression instanceof Operator.NotIn)
    return [...collect(expression.left), ...collect(expression.right)];
  if (
    (expression instanceof Operator.Between || expression instanceof Operator.NotBetween) &&
    expression.value instanceof Identifier &&
    expression.low instanceof Literal &&
    expression.high instanceof Literal
  )
    return [
      { name: `${parameterName(expression.value.name)}_0`, value: expression.low.value },
      { name: `${parameterName(expression.value.name)}_1`, value: expression.high.value },
    ];
  if (expression instanceof Operator.Between || expression instanceof Operator.NotBetween)
    return [...collect(expression.value), ...collect(expression.low), ...collect(expression.high)];
  const operands = expression instanceof Operator ? pair(expression) : undefined;
  if (operands && operands.left instanceof Identifier && operands.right instanceof Literal)
    return [{ name: parameterName(operands.left.name), value: operands.right.value }];
  if (operands && operands.left instanceof Literal && operands.right instanceof Literal) return [];
  if (operands) return [...collect(operands.left), ...collect(operands.right)];
  if (expression instanceof ArrayExpression) return expression.elements.flatMap(collect);
  throw new Error("empty expression");
};

export class Where implements IWhere {
  readonly condition: IExpression;

  constructor(condition: IExpression);
  constructor(column: string, op: Comparison, value: Cell);
  constructor(columnOrCondition: IExpression | string, op: Comparison = eq, value: Cell = null) {
    this.condition =
      typeof columnOrCondition === "string" ? op(columnOrCondition, value) : columnOrCondition;
  }

  and(column: string, op: Comparison, value: Cell): Where {
    return new Where(new Operator.And(this.condition, op(column, value)));
  }

  or(column: string, op: Comparison, value: Cell): Where {
    return new Where(new Operator.Or(this.condition, op(column, value)));
  }

  map(column: (name: string) => string, value: (name: string, cell: Cell) => Cell): Where {
    return new Where(rewrite(this.condition, column, value));
  }

  accept = (visitor: IConditionVisitor): string => visitor.visit(this);

  get parameters(): Parameter[] {
    return collect(this.condition) as Parameter[];
  }
}
