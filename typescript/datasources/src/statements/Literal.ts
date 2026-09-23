import type { Cell } from "../converters/Cell.ts";
import type { IExpression } from "./IExpression.ts";

export class Literal implements IExpression {
  constructor(readonly value: Cell) {}
}
