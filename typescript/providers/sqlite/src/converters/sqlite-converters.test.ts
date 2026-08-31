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
    expect(Object.keys(DefaultTypeConverters.for("sqlite"))).toEqual(
      Object.keys(SqliteTypes),
    );
    expect(DefaultTypeConverters.for("postgres")).toEqual({});
  });

  it("maps bool", () => {
    const converter = new BoolConverter();
    expect(converter.convert(1)).toBe(true);
    expect(converter.convert(0)).toBe(false);
    expect(converter.convert("x")).toBe("x");
    expect(converter.reverseConvert(true)).toBe(1);
    expect(converter.reverseConvert(false)).toBe(0);
    expect(converter.reverseConvert("x")).toBe("x");
  });

  it("maps datetime", () => {
    const converter = new DateTimeConverter();
    const date = new Date("2020-01-02T03:04:05.000Z");
    expect(converter.convert("2020-01-02T03:04:05.000Z")).toEqual(date);
    expect(converter.convert(date)).toBe(date);
    expect(converter.reverseConvert(date)).toBe("2020-01-02T03:04:05.000Z");
    expect(converter.reverseConvert("x")).toBe("x");
  });

  it("maps uuid", () => {
    const converter = new UuidConverter();
    const id = "550e8400-e29b-41d4-a716-446655440000";
    expect(converter.convert(id)).toBe(id);
    expect(converter.reverseConvert(id)).toBe(id);
  });

  it("maps decimal", () => {
    const converter = new DecimalConverter();
    expect(converter.convert("10.5")).toBe("10.5");
    expect(converter.reverseConvert(10.5)).toBe("10.5");
    expect(converter.reverseConvert("10.5")).toBe("10.5");
  });

  it("maps binary", () => {
    const converter = new BinaryConverter();
    const bytes = new Uint8Array([1, 2]);
    expect(converter.convert(bytes)).toBe(bytes);
    expect(converter.reverseConvert(bytes)).toBe(bytes);
  });
});
