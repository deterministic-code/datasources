import type { Cells } from "./Cell.ts";
import type { Constructable } from "./Constructable.ts";

export abstract class TypeConverter {
  abstract convert<T extends Cells>(row: Cells, type?: Constructable<T>): T;
  abstract reverseConvert<T extends Cells>(row: T, type?: Constructable<T>): Cells;
}
