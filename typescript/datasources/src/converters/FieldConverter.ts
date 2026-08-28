import type { Cell } from "./Cell.ts";

export type FieldConverter = {
  to: (value: Cell) => Cell;
  from: (value: Cell) => Cell;
};
