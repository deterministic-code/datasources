import type { Parameter } from "../Parameter.ts";
import type { IText } from "./IText.ts";

export interface IStatement extends IText {
  readonly parameters?: Parameter[];
}
