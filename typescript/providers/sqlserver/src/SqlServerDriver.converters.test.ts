import { describe, expect, it } from "vitest";
import { SqlServerDriver } from "./SqlServerDriver.ts";

describe("SqlServerDriver converters", () => {
  it("maps required decimals and defaults", () => {
    const driver = new SqlServerDriver();
    expect(
      driver.typeConverter.convertField({ type: "decimal", name: "amount", size: [10, 2] }),
    ).toBe("DECIMAL(10, 2)");
    expect(driver.typeConverter.convert("binary")).toBe("VARBINARY(MAX)");
    expect(driver.defaultValueConverter.convert("datetime", "UtcNow")).toBe("GETUTCDATE()");
    expect(driver.typeConverter.convertField({ type: "string", size: 64 })).toBe("NVARCHAR(64)");
    expect(driver.typeConverter.convertField({ type: "binary", size: 16 })).toBe("VARBINARY(16)");
    expect(driver.defaultValueConverter.convert("uuid", "NewId")).toBe("NEWID()");
    expect(driver.defaultValueConverter.convert("datetime", "Now")).toBe("GETDATE()");
    expect(driver.defaultValueConverter.convert("binary", "Hex('ab')")).toBe("0xab");
    expect(driver.typeConverter.convert("integer")).toBe("INT");
    expect(driver.typeConverter.convert("uuid")).toBe("UNIQUEIDENTIFIER");
    expect(driver.typeConverter.convert("datetime")).toBe("DATETIME2");
    expect(driver.defaultValueConverter.convert("boolean", "true")).toBe("1");
    expect(driver.defaultValueConverter.reverseConvert("uuid", "NEWID()")).toBe("NewId");
    expect(driver.defaultValueConverter.reverseConvert("binary", "0xab")).toBe("Hex('ab')");
    expect(Object.keys(driver.rowTypeConverters)).toEqual([]);
    expect(() => driver.typeConverter.convert("nope")).toThrow(/Unknown spec field type/);
  });
});

