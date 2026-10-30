import { describe, expect, it } from "vitest";
import { OracleDriver } from "./OracleDriver.ts";

describe("OracleDriver converters", () => {
  it("maps sized types and defaults", () => {
    const driver = new OracleDriver();
    expect(driver.typeConverter.convertField({ type: "string", size: 64 })).toBe("VARCHAR2(64)");
    expect(driver.typeConverter.convertField({ type: "character", size: 4 })).toBe("CHAR(4)");
    expect(driver.typeConverter.convertField({ type: "decimal", name: "amount", size: [10, 2] })).toBe("NUMBER(10, 2)");
    expect(driver.typeConverter.convertField({ type: "binary", size: 16 })).toBe("RAW(16)");
    expect(driver.defaultValueConverter.convert("uuid", "NewId")).toBe("SYS_GUID()");
    expect(driver.defaultValueConverter.convert("datetime", "Now")).toBe("LOCALTIMESTAMP");
    expect(driver.defaultValueConverter.convert("datetime", "UtcNow")).toBe("SYS_EXTRACT_UTC(SYSTIMESTAMP)");
    expect(driver.defaultValueConverter.convert("binary", "Hex('ab')")).toBe("HEXTORAW('ab')");
  });
});
