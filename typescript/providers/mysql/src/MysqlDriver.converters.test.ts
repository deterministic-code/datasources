import { describe, expect, it } from "vitest";
import { MysqlDriver } from "./MysqlDriver.ts";

describe("MysqlDriver converters", () => {
  it("maps oversized strings to LONGTEXT", () => {
    const driver = new MysqlDriver();
    expect(driver.typeConverter.convert({ type: "string", size: 20000 })).toBe(
      "LONGTEXT",
    );
    expect(driver.defaultValueConverter.convert("uuid", "NewId")).toBe("(UUID())");
    expect(driver.typeConverter.convert({ type: "string", size: 64 })).toBe("VARCHAR(64)");
    expect(driver.typeConverter.convert({ type: "character", size: 4 })).toBe("CHAR(4)");
    expect(driver.typeConverter.convert({ type: "binary", size: 16 })).toBe("VARBINARY(16)");
    expect(driver.typeConverter.convert({ type: "binary", size: 70000 })).toBe("LONGBLOB");
    expect(driver.defaultValueConverter.convert("datetime", "Now")).toBe("(NOW(3))");
    expect(driver.defaultValueConverter.convert("datetime", "UtcNow")).toBe("(UTC_TIMESTAMP(3))");
    expect(driver.defaultValueConverter.convert("binary", "Hex('ab')")).toBe("(X'ab')");
    expect(driver.defaultValueConverter.reverseConvert("binary", "(X'ab')")).toBe("Hex('ab')");
    expect(driver.defaultValueConverter.reverseConvert("binary", "X'ab'")).toBe("Hex('ab')");
    expect(() => driver.defaultValueConverter.reverseConvert("binary", "X'ab')")).toThrow(
      /Native default/,
    );
    expect(() => driver.defaultValueConverter.reverseConvert("binary", "(X'ab'")).toThrow(
      /Native default/,
    );
    expect(driver.defaultValueConverter.convert("string", "")).toBe("('')");
    expect(driver.defaultValueConverter.convert("string", "hi")).toBe("('hi')");
    expect(driver.defaultValueConverter.reverseConvert("string", "('hi')")).toBe("hi");
    expect(driver.defaultValueConverter.reverseConvert("string", "'hi'")).toBe("hi");
    expect(() => driver.defaultValueConverter.reverseConvert("string", "'hi')")).toThrow(
      /Native default/,
    );
    expect(() => driver.defaultValueConverter.reverseConvert("string", "('hi'")).toThrow(
      /Native default/,
    );
  });
});
