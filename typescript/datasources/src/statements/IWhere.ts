import type { Parameter } from "../Parameter.ts";
import type { IExpression } from "./IExpression.ts";
import type { IText } from "./IText.ts";

export interface IWhere extends IExpression, IText {
  readonly parameters?: Parameter[];
}
