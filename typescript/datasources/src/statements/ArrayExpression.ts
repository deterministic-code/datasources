import type { IExpression } from "./IExpression.ts";
import { brand } from "./brand.ts";

export class ArrayExpression implements IExpression {
  constructor(readonly elements: IExpression[]) {}
}

brand(ArrayExpression);
