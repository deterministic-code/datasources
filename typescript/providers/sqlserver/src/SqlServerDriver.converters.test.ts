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
  });
});
