import { describe, expect, it } from "vitest";
import type { Cell } from "./Cell.ts";
import { TypeConverter } from "./TypeConverter.ts";
import { MetaDataRowTypeConverter } from "./MetaDataRowTypeConverter.ts";
import { RowTypeConverter } from "./RowTypeConverter.ts";

class OnlyOne extends TypeConverter<Cell, Cell> {
  convert = (value: Cell): Cell => value === 1;
  reverseConvert = (value: Cell): Cell => value;
}

describe("MetaDataRowTypeConverter", () => {
  it("convert and reverseConvert round trip", () => {
    const converter = new MetaDataRowTypeConverter();
    const item = converter.convert({ id: 7, username: "ada" });
    const row = converter.reverseConvert(item);
    expect(item).toEqual({ id: 7, username: "ada" });
    expect(row).toEqual({ id: 7, username: "ada" });
  });

  it("lets field converters override", () => {
    const converter = new MetaDataRowTypeConverter({ flag: new OnlyOne() });
    const dest = converter.convert({ flag: 1, name: "a" });
    expect(dest.flag).toBe(true);
    expect(dest.name).toBe("a");
    expect(converter.reverseConvert(dest).flag).toBe(true);
  });

  it("can construct a typed destination", () => {
    const converter = new MetaDataRowTypeConverter({ flag: new OnlyOne() });
    class Item {
      flag = false;
    }
    expect(converter.convert({ flag: 1 }, Item)).toBeInstanceOf(Item);
  });

  it("keeps a null in a string field", () => {
    class Item {
      name = "";
    }
    const converter = new MetaDataRowTypeConverter<Item>();
    expect(converter.convert({ name: null }, Item).name).toBeNull();
  });

  it("copies a reference id onto the nested object", () => {
    class Item {
      Type: { Id?: number; id?: number } = { Id: 0 };
    }
    const converter = new MetaDataRowTypeConverter<Item>();
    converter.references.add("Type");
    const nested = converter.convert({ Type: 4 }, Item);
    expect(nested.Type).toEqual({ Id: 4 });
    class Bare {
      Type = 0;
    }
    const bare = new MetaDataRowTypeConverter<Bare>();
    bare.references.add("Type");
    expect(bare.convert({ Type: 4 }, Bare).Type).toEqual({ id: 4 });
    expect(converter.reverseConvert(Object.assign(new Item(), { Type: { Id: 3 } })).Type).toBe(3);
    expect(converter.reverseConvert(Object.assign(new Item(), { Type: { id: 5 } })).Type).toBe(5);
    expect(converter.reverseConvert(Object.assign(new Item(), { Type: 9 as never })).Type).toBe(9);
    expect(converter.reverseConvert(Object.assign(new Item(), { Type: undefined as never })).Type).toBeUndefined();
    class Dated {
      created = new Date(0);
      blob = new Uint8Array();
    }
    const dated = new MetaDataRowTypeConverter<Dated>();
    const created = new Date("2024-01-15T10:00:00.000Z");
    const blob = new Uint8Array([1, 2]);
    expect(dated.reverseConvert(Object.assign(new Dated(), { created, blob }))).toEqual({ created, blob });
    class Flip extends TypeConverter<Cell, Cell> {
      convert = (value: Cell): Cell => value;
      reverseConvert = (): Cell => 7;
    }
    const flagged = new MetaDataRowTypeConverter<Item>({ Type: new Flip() });
    flagged.references.add("Type");
    expect(flagged.reverseConvert(Object.assign(new Item(), { Type: { Id: 1 } })).Type).toBe(7);
  });
});

describe("RowTypeConverter", () => {
  it("converts named columns and leaves the rest", () => {
    const flag = new OnlyOne();
    const converter = RowTypeConverter.from({ boolean: flag }, { on: "boolean", missing: "nope" });
    expect(converter.convert({ on: 1, name: "a" })).toEqual({ on: true, name: "a" });
    expect(converter.reverseConvert({ on: true, name: "a" })).toEqual({ on: true, name: "a" });
    const ages = RowTypeConverter.from({ integer: flag }, { age: "unsignedinteger" });
    expect(ages.convert({ age: 1 })).toEqual({ age: true });
  });
});
