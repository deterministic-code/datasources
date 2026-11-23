import { describe, expect, it } from "vitest";
import { RowTypeConverter } from "@deterministic-code/datasources";
import { MysqlDriver } from "../MysqlDriver.ts";
import { reverse, MysqlRowTypeConverters } from "./MysqlRowTypeConverters.ts";

describe("MysqlRowTypeConverters", () => {
  it("registers default types", () => {
    expect(Object.keys(new MysqlDriver().rowTypeConverters)).toEqual(Object.keys(MysqlRowTypeConverters));
  });


  it("converts mapping-cell driver cells", () => {
    const opened = new Date("2020-01-02T03:04:05.000Z");
    const blob = new Uint8Array([1, 2, 3, 4]);
    const latin1 = Buffer.from(blob).toString("latin1");
    const converter = RowTypeConverter.from(MysqlRowTypeConverters, {
      textToInteger: "integer",
      textToDecimal: "decimal",
      textToDatetime: "datetime",
      textToBinary: "binary",
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
    });
    expect(
      converter.convert({
        textToInteger: "10",
        textToDecimal: "10.50",
        textToDatetime: "2020-01-02T03:04:05.000Z",
        textToBinary: latin1,
        intToInteger: 10,
        intToFloat: 10,
        floatToInteger: 10.0,
        floatToFloat: 10.9,
        floatToDecimal: 10.5,
        decimalToFloat: 1.2345678901234567,
        decimalToDecimal: "10.50000000000000",
        datetimeToDatetime: opened,
        intToBoolean: 1,
      }),
    ).toEqual({
      textToInteger: 10,
      textToDecimal: "10.5",
      textToDatetime: opened,
      textToBinary: blob,
      intToInteger: 10,
      intToFloat: 10,
      floatToInteger: 10,
      floatToFloat: 10.9,
      floatToDecimal: "10.5",
      decimalToFloat: 1.2345678901234567,
      decimalToDecimal: "10.5",
      datetimeToDatetime: opened,
      intToBoolean: true,
    });
    expect(converter.convert({ decimalToFloat: "1.23456789012346" })).toEqual({
      decimalToFloat: "1.23456789012346",
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
        floatToInteger: 10,
        textToBinary: blob,
        datetimeToDatetime: opened,
        decimalToDecimal: "10.50",
        floatToDecimal: 10.5,
      }),
    ).toEqual({
      intToInteger: 10n,
      floatToInteger: 10n,
      textToBinary: blob,
      datetimeToDatetime: "2020-01-02 03:04:05.000",
      decimalToDecimal: "10.5",
      floatToDecimal: "10.5",
    });
  });

  it("reverses booleans, dates, and bytes", () => {
    const when = new Date("2024-01-02T00:00:00.000Z");
    expect(reverse(true)).toBe(1);
    expect(reverse(false)).toBe(0);
    expect(reverse(when)).toBe(when.toISOString().replace("T", " ").replace("Z", ""));
    expect(reverse(new Uint8Array([1, 2]))).toEqual(new Uint8Array([1, 2]));
    expect(reverse("a")).toBe("a");
    expect(reverse("2024-01-02T00:00:00Z")).toBe("2024-01-02T00:00:00Z");
  });
});
