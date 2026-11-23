import { describe, expect, it } from "vitest";
import { RowTypeConverter, Schema } from "@deterministic-code/datasources";
import { PostgresDriver } from "../PostgresDriver.ts";
import { reverse, PostgresRowTypeConverters } from "./PostgresRowTypeConverters.ts";

const floorSchema = () => new Schema({ floor: { type: "unsignedinteger", nullable: true, hasDefault: false } });

describe("PostgresRowTypeConverters", () => {
  it("registers default types", () => {
    expect(Object.keys(new PostgresDriver().rowTypeConverters)).toEqual(Object.keys(PostgresRowTypeConverters));
  });

  it("converts BIGINT text floor through unsignedinteger", () => {
    const types = floorSchema().types();
    expect(RowTypeConverter.from({ integer: PostgresRowTypeConverters.integer }, types).convert({ floor: "2" })).toEqual({
      floor: "2",
    });
    expect(RowTypeConverter.from(PostgresRowTypeConverters, types).convert({ floor: "2" })).toEqual({ floor: 2 });
  });

  it("converts mapping-cell driver cells", () => {
    const opened = new Date("2020-01-02T03:04:05.000Z");
    const converter = RowTypeConverter.from(PostgresRowTypeConverters, {
      textToInteger: "integer",
      textToDecimal: "decimal",
      textToDatetime: "datetime",
      floatToInteger: "integer",
      decimalToInteger: "integer",
      decimalToDecimal: "decimal",
      datetimeToDatetime: "datetime",
      intToBoolean: "boolean",
    });
    expect(
      converter.convert({
        textToInteger: "10",
        textToDecimal: "10.50",
        textToDatetime: "2020-01-02T03:04:05.000Z",
        floatToInteger: 10.0,
        decimalToDecimal: "10.50000000000000",
        datetimeToDatetime: opened,
        intToBoolean: 1,
      }),
    ).toEqual({
      textToInteger: 10,
      textToDecimal: "10.5",
      textToDatetime: opened,
      floatToInteger: 10,
      decimalToDecimal: "10.5",
      datetimeToDatetime: opened,
      intToBoolean: true,
    });
    expect(converter.convert({ textToInteger: "10.0" })).toEqual({ textToInteger: 10 });
    expect(converter.convert({ floatToInteger: "10.0" })).toEqual({ floatToInteger: 10 });
    expect(converter.convert({ decimalToInteger: "10.00000000000000" })).toEqual({
      decimalToInteger: 10,
    });
    expect(() => converter.convert({ decimalToInteger: "10.50000000000000" })).toThrow(/is not a long/);
    expect(
      converter.reverseConvert({
        datetimeToDatetime: opened,
        decimalToDecimal: "10.50",
      }),
    ).toEqual({
      datetimeToDatetime: "2020-01-02T03:04:05.000Z",
      decimalToDecimal: "10.5",
    });
  });

  it("reverses booleans, dates, and bytes", () => {
    const when = new Date("2024-01-02T00:00:00.000Z");
    expect(reverse(true)).toBe(true);
    expect(reverse(false)).toBe(false);
    expect(reverse(when)).toBe("2024-01-02T00:00:00.000Z");
    expect(reverse(new Uint8Array([1, 2]))).toEqual(new Uint8Array([1, 2]));
    expect(reverse("a")).toBe("a");
  });
});
