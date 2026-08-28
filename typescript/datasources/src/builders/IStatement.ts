import type { Parameter } from "../Parameter.ts";

export interface IStatement {
  getText(): string;
  readonly parameters?: Parameter[];
}
