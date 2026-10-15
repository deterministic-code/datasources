import { describe, expect, it } from "vitest";
import { SqlServerDriver } from "./SqlServerDriver.ts";

describe("SqlServerDriver converters", () => {
  it("maps required decimals and defaults", () => {
    const driver = new SqlServerDriver();
    expect(
      driver.typeConverter.convert({ type: "decimal", name: "amount", size: [10, 2] }),
    ).toBe("DECIMAL(10, 2)");
    expect(driver.typeConverter.convert("binary")).toBe("VARBINARY(MAX)");
    expect(driver.defaultValueConverter.convert("datetime", "UtcNow")).toBe("GETUTCDATE()");
  });
});
