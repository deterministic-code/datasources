import type { Cells } from "./Cell.ts";

export type TypeMapping<T> = {
  name: string;
  source: string;
  destination: "typescript";
  schema: Record<string, string>;
  to: (row: Cells) => T;
  from: (obj: T) => Cells;
};
