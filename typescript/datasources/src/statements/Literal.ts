import type { Value } from "../converters/Value.ts";
import type { IExpression } from "./IExpression.ts";
import { brand } from "./brand.ts";

export class Literal implements IExpression {
  constructor(readonly value: Value) {}
}

brand(Literal);
