import type { Cell, Cells } from "./Cell.ts";
import { TypeConverter } from "./TypeConverter.ts";
import type { Constructable } from "./Constructable.ts";

export class MetaDataRowTypeConverter<T = Cells> extends TypeConverter<T, Cells> {
  types: Record<string, TypeConverter<Cell, Cell>>;
  fields: Record<string, TypeConverter<Cell, Cell>>;

  constructor(
    types: Record<string, TypeConverter<Cell, Cell>> = {},
    fields: Record<string, TypeConverter<Cell, Cell>> = {},
  ) {
    super();
    this.types = { ...types };
    this.fields = { ...fields };
  }

  convert = (row: Cells, type?: Constructable<T>): T => {
    const blank = type ? new type() : {};
    const cells = Object.fromEntries(
      Object.entries(row).filter(([key, value]) => {
        const current = (blank as Cells)[key];
        return !(typeof current === "string" && typeof value !== "string" && value !== null);
      }),
    );
    const dest = Object.assign(blank, cells) as T;
    for (const [key, value] of Object.entries(row)) {
      const converter = this.fields[key];
      if (converter) (dest as Cells)[key] = converter.convert(value);
    }
    return dest;
  };

  reverseConvert = (row: T): Cells =>
    Object.fromEntries(
      Object.entries(row as Cells)
        .filter(([, value]) => !Array.isArray(value))
        .map(([key, value]) => {
          const converter = this.fields[key];
          return [key, converter ? converter.reverseConvert(value) : value];
        }),
    );
}
