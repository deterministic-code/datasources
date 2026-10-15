import { describe, expect, it } from "vitest";
import { MysqlDriver } from "./MysqlDriver.ts";

describe("MysqlDriver converters", () => {
  it("maps oversized strings to LONGTEXT", () => {
    const driver = new MysqlDriver();
    expect(driver.typeConverter.convert({ type: "string", size: 20000 })).toBe(
      "LONGTEXT",
    );
    expect(driver.defaultValueConverter.convert("uuid", "NewId")).toBe("(UUID())");
  });
});
