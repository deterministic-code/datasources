import type { IExpression } from "./IExpression.ts";

export class Identifier implements IExpression {
  constructor(readonly name: string) {}
}
