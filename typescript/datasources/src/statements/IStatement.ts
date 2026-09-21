import type { Parameter } from "../Parameter.ts";
import type { IExpression } from "./IExpression.ts";

export interface IStatement extends IExpression {
  getText(): string;
  readonly parameters?: Parameter[];
}
