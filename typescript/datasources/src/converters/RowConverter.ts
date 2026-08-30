import type { Cells } from "./Cell.ts";
import type { Constructable } from "./Constructable.ts";

export class RowConverter {
  convert = <T extends Cells>(row: Cells, type?: Constructable<T>): T =>
    Object.assign(type ? new type() : {}, row) as T;
  reverseConvert = <T extends Cells>(obj: T): Cells => ({ ...obj });
  static keys = (type?: Constructable<Cells>): string[] =>
    type
      ? Object.entries(new type() as object)
          .filter(([, value]) => value !== undefined)
          .map(([key]) => key)
      : [];
}
