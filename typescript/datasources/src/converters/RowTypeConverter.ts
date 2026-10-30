import type { Cell, Cells } from "./Cell.ts";
import { TypeConverter } from "./TypeConverter.ts";

export class RowTypeConverter extends TypeConverter<Cells, Cells> {
  constructor(readonly fields: Record<string, TypeConverter<Cell, Cell>>) {
    super();
  }

  static from(
    converters: Record<string, TypeConverter<Cell, Cell>>,
    columns: Record<string, string>,
  ): RowTypeConverter {
    const fields: Record<string, TypeConverter<Cell, Cell>> = {};
    for (const [name, type] of Object.entries(columns)) {
      const converter = converters[type] ?? (type.endsWith("integer") ? converters.integer : undefined);
      if (converter) fields[name] = converter;
    }
    return new RowTypeConverter(fields);
  }

  convert = (from: Cells): Cells => this.apply(from, false);

  reverseConvert = (to: Cells): Cells => this.apply(to, true);

  private apply(row: Cells, reverse: boolean): Cells {
    return Object.fromEntries(
      Object.entries(row).map(([name, value]) => {
        const converter = this.fields[name];
        return [name, converter ? (reverse ? converter.reverseConvert(value) : converter.convert(value)) : value];
      }),
    );
  }
}
