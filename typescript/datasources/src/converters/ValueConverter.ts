import type { Cell } from "./Cell.ts";

export type ValueConverter = {
  convert: (value: Cell) => Cell;
  reverseConvert: (value: Cell) => Cell;
};
