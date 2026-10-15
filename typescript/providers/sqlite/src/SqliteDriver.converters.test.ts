import { describe, expect, it } from "vitest";
import { SqliteDriver } from "./SqliteDriver.ts";

describe("SqliteDriver converters", () => {
  it("maps UtcNow and sized strings", () => {
    const driver = new SqliteDriver();
    expect(driver.defaultValueConverter.convert("datetime", "UtcNow")).toBe(
      `(strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))`,
    );
    expect(driver.typeConverter.convert({ type: "string", size: 64 })).toBe(
      "VARCHAR(64)",
    );
    expect(driver.typeConverter.convert({ type: "uuid" })).toBe("TEXT");
  });
});
