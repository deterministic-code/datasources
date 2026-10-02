import type { Where } from "./Where.ts";
import type { WhereText } from "./WhereText.ts";

export interface IConditionVisitor {
  visit(where: Where): string;
  visit(where: WhereText): string;
}
