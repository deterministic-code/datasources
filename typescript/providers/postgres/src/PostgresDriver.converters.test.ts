import { describe, expect, it } from "vitest";
import { PostgresDriver } from "./PostgresDriver.ts";

describe("PostgresDriver converters", () => {
  it("maps NewId and boolean defaults", () => {
    const driver = new PostgresDriver();
    expect(driver.defaultValueConverter.convert("uuid", "NewId")).toBe("gen_random_uuid()");
    expect(driver.defaultValueConverter.convert("boolean", "true")).toBe("TRUE");
    expect(driver.typeConverter.convert({ type: "datetime" })).toBe("TIMESTAMPTZ");
  });
});
