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
  it("convert and reverseConvert round trip", () => {
    const converter = new RowTypeConverter();
    const item = converter.convert({ id: 7, username: "ada" });
    const row = converter.reverseConvert(item);
    expect(item).toEqual({ id: 7, username: "ada" });
    expect(row).toEqual({ id: 7, username: "ada" });
  });

  it("lets field converters override", () => {
    const converter = new RowTypeConverter({}, { flag: new OnlyOne() });
    const dest = converter.convert({ flag: 1, name: "a" });
    expect(dest.flag).toBe(true);
    expect(dest.name).toBe("a");
    expect(converter.reverseConvert(dest).flag).toBe(true);
  });

  it("can construct a typed destination", () => {
    const converter = new RowTypeConverter({}, { flag: new OnlyOne() });
    class Item {
      flag = false;
    }
    expect(converter.convert({ flag: 1 }, Item)).toBeInstanceOf(Item);
  });

  it("registers an empty provider catalog", () => {
    DefaultTypeConverters.use("memory", {});
    expect(DefaultTypeConverters.for("memory")).toEqual({});
  });
});
