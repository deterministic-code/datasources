import { describe, expect, it } from "vitest";
import { PostgresDriver } from "./PostgresDriver.ts";

describe("PostgresDriver converters", () => {
  it("maps NewId and boolean defaults", () => {
    const driver = new PostgresDriver();
    expect(driver.defaultValueConverter.convert("uuid", "NewId")).toBe("gen_random_uuid()");
    expect(driver.defaultValueConverter.convert("boolean", "true")).toBe("TRUE");
    expect(driver.typeConverter.convertField({ type: "datetime" })).toBe("TIMESTAMPTZ");
    expect(driver.typeConverter.convertField({ type: "string", size: 64 })).toBe("VARCHAR(64)");
    expect(driver.typeConverter.convertField({ type: "character", size: 4 })).toBe("CHAR(4)");
    expect(driver.defaultValueConverter.convert("datetime", "Now")).toBe("LOCALTIMESTAMP");
    expect(driver.defaultValueConverter.convert("datetime", "UtcNow")).toBe(`(NOW() AT TIME ZONE 'UTC')`);
    expect(driver.defaultValueConverter.convert("binary", "Hex('ab')")).toBe(`'\\xab'`);
    expect(driver.typeConverter.convert("integer")).toBe("INTEGER");
    expect(driver.typeConverter.convert("unsignedinteger")).toBe("BIGINT");
    expect(driver.typeConverter.convert("uuid")).toBe("UUID");
    expect(driver.typeConverter.convert("boolean")).toBe("BOOLEAN");
    expect(driver.typeConverter.reverseConvert("INTEGER")).toBe("integer");
    expect(driver.typeConverter.reverseConvert("BIGINT")).toBe("unsignedinteger");
    expect(driver.defaultValueConverter.convert("uuid", "Empty")).toContain("00000000");
    expect(driver.defaultValueConverter.reverseConvert("uuid", "gen_random_uuid()")).toBe("NewId");
    expect(driver.defaultValueConverter.reverseConvert("binary", `'\\xab'`)).toBe("Hex('ab')");
    expect(() => driver.typeConverter.convert("nope")).toThrow(/Unknown spec field type/);
    expect(() => driver.defaultValueConverter.convert("nope", "x")).toThrow(/Unknown spec field type/);
  });
});

