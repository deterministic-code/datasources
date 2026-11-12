import { describe, expect, it } from "vitest";
import { emptyDefaultValueConverter } from "@deterministic-code/datasources";
import { JsonDriver } from "./JsonDriver.ts";

describe("JsonDriver converters", () => {
  it("leaves type and row converters empty", () => {
    const driver = new JsonDriver();
    expect(driver.typeConverter).toBeUndefined();
    expect(driver.rowTypeConverters).toEqual({});
    expect(driver.defaultValueConverter).toBe(emptyDefaultValueConverter);
  });
});
