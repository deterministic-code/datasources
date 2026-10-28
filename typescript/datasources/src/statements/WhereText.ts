import type { IConditionVisitor } from "./IConditionVisitor.ts";
import type { IWhere } from "./IWhere.ts";
import { brand } from "./brand.ts";

export class WhereText implements IWhere {
  constructor(private readonly text: string) {}
  get rawText(): string {
    return this.text;
  }
  accept = (visitor: IConditionVisitor): string => visitor.visit(this);
}

brand(WhereText);
