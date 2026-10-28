import type { Cell } from "../converters/Cell.ts";
import type { IExpression } from "./IExpression.ts";
import { brand } from "./brand.ts";

export class Literal implements IExpression {
  constructor(readonly value: Cell) {}
}

brand(Literal);
