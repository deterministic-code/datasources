import { describe, expect, it } from "vitest";
import { PostgresDriver } from "./PostgresDriver.ts";

describe("PostgresDriver converters", () => {
  it("maps NewId and boolean defaults", () => {
    const driver = new PostgresDriver();
    expect(driver.defaultValueConverter.convert("uuid", "NewId")).toBe("gen_random_uuid()");
    expect(driver.defaultValueConverter.convert("boolean", "true")).toBe("TRUE");
    expect(driver.typeConverter.convert({ type: "datetime" })).toBe("TIMESTAMPTZ");
    expect(driver.typeConverter.convert({ type: "string", size: 64 })).toBe("VARCHAR(64)");
    expect(driver.typeConverter.convert({ type: "character", size: 4 })).toBe("CHAR(4)");
    expect(driver.defaultValueConverter.convert("datetime", "Now")).toBe("LOCALTIMESTAMP");
    expect(driver.defaultValueConverter.convert("datetime", "UtcNow")).toBe(`(NOW() AT TIME ZONE 'UTC')`);
    expect(driver.defaultValueConverter.convert("binary", "Hex('ab')")).toBe(`'\\xab'`);
  });
});
