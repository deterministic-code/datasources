import type { IExpression } from "./IExpression.ts";
import { brand } from "./brand.ts";

export class Identifier implements IExpression {
  constructor(readonly name: string) {}
}

brand(Identifier);
