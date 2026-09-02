import { describe, expect, it } from "vitest";
import type { Cell } from "./Cell.ts";
import { TypeConverter } from "./TypeConverter.ts";
import { DefaultTypeConverters } from "./DefaultTypeConverters.ts";
import { RowTypeConverter } from "./RowTypeConverter.ts";

class OnlyOne extends TypeConverter<Cell, Cell> {
  convert = (value: Cell): Cell => value === 1;
  reverseConvert = (value: Cell): Cell => value;
}

describe("RowTypeConverter", () => {
  it("lets field converters override", () => {
    const converter = new RowTypeConverter({}, { flag: new OnlyOne() });
    expect(converter.convert({ flag: 2 }).flag).toBe(false);
    expect(converter.convert({ flag: 1 }).flag).toBe(true);
    class Item {
      flag = false;
    }
    expect(converter.convert({ flag: 1 }, Item)).toBeInstanceOf(Item);
    expect(converter.reverseConvert({ flag: true }).flag).toBe(true);
    DefaultTypeConverters.use("memory", {});
    expect(DefaultTypeConverters.for("memory")).toEqual({});
  });
});


