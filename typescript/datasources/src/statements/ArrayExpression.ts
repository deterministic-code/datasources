import type { IExpression } from "./IExpression.ts";

export class ArrayExpression implements IExpression {
  constructor(readonly elements: IExpression[]) {}
}
