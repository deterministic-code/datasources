import type { Cell, Cells } from "./Cell.ts";
import { TypeConverter } from "./TypeConverter.ts";
import type { IDriver } from "../IDriver.ts";

export class RowTypeConverter extends TypeConverter<Cells, Cells> {
  constructor(readonly fields: Record<string, TypeConverter<Cell, Cell>>) {
    super();
  }

  static from(driver: IDriver, columns: Record<string, string>): RowTypeConverter {
    const native = nativeTypes(driver);
    const fields: Record<string, TypeConverter<Cell, Cell>> = {};
    for (const [name, type] of Object.entries(columns)) {
      const converter = driver.rowTypeConverters[type] ?? native[type.toLowerCase()];
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

const nativeTypes = (driver: IDriver): Record<string, TypeConverter<Cell, Cell>> => {
  const groups = new Map<string, TypeConverter<Cell, Cell>[]>();
  for (const [name, converter] of Object.entries(driver.rowTypeConverters)) {
    const sql = driver.columnType(name).toLowerCase();
    groups.set(sql, [...(groups.get(sql) ?? []), converter]);
  }
  const native: Record<string, TypeConverter<Cell, Cell>> = {};
  for (const [sql, converters] of groups) {
    const [only] = converters;
    if (converters.length === 1 && only) native[sql] = only;
  }
  return native;
};
