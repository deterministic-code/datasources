import { describe, expect, it } from "vitest";
import { RowTypeConverter, Schema } from "@deterministic-code/datasources";
import { SqliteDriver } from "../SqliteDriver.ts";
import { reverse, SqliteRowTypeConverters } from "./SqliteRowTypeConverters.ts";

const floorSchema = () => new Schema({ floor: { type: "unsignedinteger", nullable: true, hasDefault: false } });

describe("SqliteRowTypeConverters", () => {
  it("registers default types", () => {
    expect(Object.keys(new SqliteDriver().rowTypeConverters)).toEqual(Object.keys(SqliteRowTypeConverters));
  });

  it("converts BIGINT text floor through unsignedinteger", () => {
    const types = floorSchema().types();
    expect(RowTypeConverter.from({ integer: SqliteRowTypeConverters.integer }, types).convert({ floor: "2" })).toEqual({
      floor: "2",
    });
    expect(RowTypeConverter.from(SqliteRowTypeConverters, types).convert({ floor: "2" })).toEqual({ floor: 2 });
  });

  it("converts mapping-cell driver cells", () => {
    const opened = new Date("2020-01-02T03:04:05.000Z");
    const token = "550e8400-e29b-41d4-a716-446655440000";
    const converter = RowTypeConverter.from(SqliteRowTypeConverters, {
      textToInteger: "integer",
      textToDecimal: "decimal",
      textToDatetime: "datetime",
      intToInteger: "integer",
      intToFloat: "float",
      intToBoolean: "boolean",
      floatToInteger: "integer",
      floatToFloat: "float",
      floatToDecimal: "decimal",
      decimalToInteger: "integer",
      decimalToFloat: "float",
      decimalToDecimal: "decimal",
      datetimeToDatetime: "datetime",
      guidToGuid: "uuid",
    });
    expect(
      converter.convert({
        textToInteger: "10",
        textToDecimal: "10.50",
        textToDatetime: "2020-01-02T03:04:05.000Z",
        intToInteger: 10,
        intToFloat: 10,
        floatToInteger: 10.0,
        floatToFloat: 10.9,
        floatToDecimal: 10.5,
        decimalToFloat: 1.2345678901234567,
        decimalToDecimal: "10.50000000000000",
        datetimeToDatetime: opened,
        intToBoolean: 1,
        guidToGuid: token,
      }),
    ).toEqual({
      textToInteger: 10,
      textToDecimal: "10.5",
      textToDatetime: opened,
      intToInteger: 10,
      intToFloat: 10,
      floatToInteger: 10,
      floatToFloat: 10.9,
      floatToDecimal: "10.5",
      decimalToFloat: 1.2345678901234567,
      decimalToDecimal: "10.5",
      datetimeToDatetime: opened,
      intToBoolean: true,
      guidToGuid: token,
    });
    expect(converter.convert({ textToInteger: "10.0" })).toEqual({ textToInteger: 10 });
    expect(converter.convert({ floatToInteger: "10.0" })).toEqual({ floatToInteger: 10 });
    expect(converter.convert({ decimalToInteger: "10.00000000000000" })).toEqual({
      decimalToInteger: 10,
    });
    expect(() => converter.convert({ decimalToInteger: "10.50000000000000" })).toThrow(/is not a long/);
    expect(
      converter.reverseConvert({
        intToInteger: 10,
        datetimeToDatetime: opened,
        decimalToDecimal: "10.50",
      }),
    ).toEqual({
      intToInteger: 10n,
      datetimeToDatetime: "2020-01-02T03:04:05.000Z",
      decimalToDecimal: "10.5",
    });
  });

  it("reverses booleans, dates, and bytes", () => {
    const when = new Date("2024-01-02T00:00:00.000Z");
    expect(reverse(true)).toBe(1);
    expect(reverse(false)).toBe(0);
    expect(reverse(when)).toBe(when.toISOString());
    expect(reverse(new Uint8Array([1, 2]))).toEqual(new Uint8Array([1, 2]));
    expect(reverse("a")).toBe("a");
  });
});
