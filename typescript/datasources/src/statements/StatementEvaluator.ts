import type { Scalar } from "../builders/Scalar.ts";
import { ArrayExpression } from "./ArrayExpression.ts";
import { Identifier } from "./Identifier.ts";
import type { IExpression } from "./IExpression.ts";
import { Literal } from "./Literal.ts";
import { Operator } from "./Operator.ts";
import { Subquery } from "./Subquery.ts";
import { Where } from "./Where.ts";
import { WhereClause } from "./WhereClause.ts";

const isTrue = (value: Scalar): boolean => value === true;

const like = (text: Scalar, pattern: Scalar): boolean =>
  text === null || pattern === null
    ? false
    : typeof text === "string" && typeof pattern === "string"
      ? wild(text, pattern)
      : (() => {
          throw new Error("empty like");
        })();

const wild = (text: string, pattern: string): boolean => {
  let t = 0;
  let p = 0;
  let starT = -1;
  let starP = -1;
  while (t < text.length) {
    if (p < pattern.length && (pattern[p] === "_" || pattern[p] === text[t])) {
      t++;
      p++;
    } else if (p < pattern.length && pattern[p] === "%") {
      starP = p++;
      starT = t;
    } else if (starP >= 0) {
      p = starP + 1;
      t = ++starT;
    } else return false;
  }
  while (p < pattern.length && pattern[p] === "%") p++;
  return p === pattern.length;
};

const between = (value: Scalar, low: Scalar, high: Scalar): boolean =>
  value === null || low === null || high === null ? false : compare(value, low) >= 0 && compare(value, high) <= 0;

const equal = (left: Scalar, right: Scalar): boolean => left === right;

const compare = (left: Scalar, right: Scalar): number =>
  typeof left === "number" && typeof right === "number"
    ? left - right
    : typeof left === "bigint" && typeof right === "bigint"
      ? Number(left - right)
      : typeof left === "string" && typeof right === "string"
        ? left < right
          ? -1
          : left > right
            ? 1
            : 0
        : (() => {
            throw new Error("empty compare");
          })();

export class StatementEvaluator {
  evaluate = (expression: IExpression, parameters?: Record<string, Scalar>): Scalar =>
    expression instanceof WhereClause
      ? this.evaluate(expression.condition, parameters)
      : expression instanceof Where
        ? (() => {
            throw new Error("text where is not evaluable");
          })()
        : expression instanceof Identifier
          ? (parameters?.[expression.name] ?? null)
          : expression instanceof Literal
            ? expression.value
            : expression instanceof Operator.Not
              ? !isTrue(this.evaluate(expression.argument, parameters))
              : expression instanceof Operator.IsNull
                ? this.evaluate(expression.argument, parameters) === null
                : expression instanceof Operator.IsNotNull
                  ? this.evaluate(expression.argument, parameters) !== null
                  : expression instanceof Operator.Like
                    ? like(this.evaluate(expression.left, parameters), this.evaluate(expression.right, parameters))
                    : expression instanceof Operator.NotLike
                      ? !like(this.evaluate(expression.left, parameters), this.evaluate(expression.right, parameters))
                      : expression instanceof Operator.Between
                        ? between(
                            this.evaluate(expression.value, parameters),
                            this.evaluate(expression.low, parameters),
                            this.evaluate(expression.high, parameters),
                          )
                        : expression instanceof Operator.NotBetween
                          ? !between(
                              this.evaluate(expression.value, parameters),
                              this.evaluate(expression.low, parameters),
                              this.evaluate(expression.high, parameters),
                            )
                          : expression instanceof Operator.NotIn
                            ? expression.right instanceof ArrayExpression
                              ? !expression.right.elements.some((element) =>
                                  equal(
                                    this.evaluate(expression.left, parameters),
                                    this.evaluate(element, parameters),
                                  ),
                                )
                              : (() => {
                                  throw new Error("empty in");
                                })()
                            : expression instanceof Operator.Exists || expression instanceof Subquery
                              ? (() => {
                                  throw new Error("subquery is not evaluable");
                                })()
                              : expression instanceof Operator.And
                ? isTrue(this.evaluate(expression.left, parameters)) &&
                  isTrue(this.evaluate(expression.right, parameters))
                : expression instanceof Operator.Or
                  ? isTrue(this.evaluate(expression.left, parameters)) ||
                    isTrue(this.evaluate(expression.right, parameters))
                  : expression instanceof Operator.In
                    ? expression.right instanceof ArrayExpression
                      ? expression.right.elements.some((element) =>
                          equal(
                            this.evaluate(expression.left, parameters),
                            this.evaluate(element, parameters),
                          ),
                        )
                      : (() => {
                          throw new Error("empty in");
                        })()
                    : expression instanceof Operator.Equals
                      ? equal(
                          this.evaluate(expression.left, parameters),
                          this.evaluate(expression.right, parameters),
                        )
                      : expression instanceof Operator.NotEquals
                        ? !equal(
                            this.evaluate(expression.left, parameters),
                            this.evaluate(expression.right, parameters),
                          )
                        : expression instanceof Operator.GreaterThan
                          ? compare(
                              this.evaluate(expression.left, parameters),
                              this.evaluate(expression.right, parameters),
                            ) > 0
                          : expression instanceof Operator.LessThan
                            ? compare(
                                this.evaluate(expression.left, parameters),
                                this.evaluate(expression.right, parameters),
                              ) < 0
                            : expression instanceof Operator.GreaterThanOrEqual
                              ? compare(
                                  this.evaluate(expression.left, parameters),
                                  this.evaluate(expression.right, parameters),
                                ) >= 0
                              : expression instanceof Operator.LessThanOrEqual
                                ? compare(
                                    this.evaluate(expression.left, parameters),
                                    this.evaluate(expression.right, parameters),
                                  ) <= 0
                                : expression instanceof ArrayExpression
                                  ? (() => {
                                      throw new Error("array is not a scalar");
                                    })()
                                  : (() => {
                                      throw new Error("empty expression");
                                    })();
}
