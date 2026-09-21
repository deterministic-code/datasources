import type { Scalar } from "../builders/Scalar.ts";
import type { IExpression } from "./IExpression.ts";

export class Literal implements IExpression {
  constructor(readonly value: Scalar) {}
}
