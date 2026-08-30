import type { Cells } from "./Cell.ts";

export type TypeMapping<T> = {
  name: string;
  source: string;
  to: (row: Cells) => T;
  from: (obj: T) => Cells;
};
