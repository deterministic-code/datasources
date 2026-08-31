import { describe, expect, it } from "vitest";
import { DefaultTypeConverters } from "@datasources/core";
import { BinaryConverter } from "./BinaryConverter.ts";
import { BoolConverter } from "./BoolConverter.ts";
import { DateTimeConverter } from "./DateTimeConverter.ts";
import { DecimalConverter } from "./DecimalConverter.ts";
import { SqliteTypes } from "./SqliteTypes.ts";
import { UuidConverter } from "./UuidConverter.ts";

describe("Sqlite converters", () => {
  it("registers default types", () => {
    expect(DefaultTypeConverters.for("sqlite").bool).toBe(SqliteTypes.bool);
  });

  it("maps integer bool", () => {
    const converter = new BoolConverter();
    expect(converter.convert(1)).toBe(true);
    expect(converter.convert(0)).toBe(false);
    expect(converter.reverseConvert(true)).toBe(1);
    expect(converter.reverseConvert(false)).toBe(0);
  });

  it("maps datetime uuid decimal binary", () => {
    expect(new DateTimeConverter().convert("2020-01-02T03:04:05.000Z")).toEqual(
      new Date("2020-01-02T03:04:05.000Z"),
    );
    expect(new UuidConverter().convert("550e8400-e29b-41d4-a716-446655440000")).toBe(
      "550e8400-e29b-41d4-a716-446655440000",
    );
    expect(new DecimalConverter().reverseConvert(10.5)).toBe("10.5");
    const bytes = new Uint8Array([1, 2]);
    expect(new BinaryConverter().convert(bytes)).toBe(bytes);
  });
});
