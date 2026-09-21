import type { Scalar } from "../builders/Scalar.ts";
import { ArrayExpression } from "./ArrayExpression.ts";
import { Identifier } from "./Identifier.ts";
import type { IExpression } from "./IExpression.ts";
import { Literal } from "./Literal.ts";
import { Operator } from "./Operator.ts";
import { Where } from "./Where.ts";
import { WhereStatement } from "./WhereStatement.ts";

const isTrue = (value: Scalar): boolean => value === true;

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
    expression instanceof WhereStatement
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
