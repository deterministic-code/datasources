import { describe, expect, it } from "vitest";
import { SqlServerDriver } from "./SqlServerDriver.ts";

describe("SqlServerDriver converters", () => {
  it("maps ROWVERSION and required decimals", () => {
    const driver = new SqlServerDriver();
    expect(
      driver.deterministicTypeConverter.convert({
        type: "binary",
        isOptimisticConcurrency: true,
        useNativeRowVersion: true,
      }),
    ).toBe("ROWVERSION");
    expect(
      driver.deterministicTypeConverter.convert({ type: "decimal", name: "amount", size: [10, 2] }),
    ).toBe("DECIMAL(10, 2)");
    expect(driver.defaultValueConverter.convert("datetime", "UtcNow")).toBe("GETUTCDATE()");
  });
});
