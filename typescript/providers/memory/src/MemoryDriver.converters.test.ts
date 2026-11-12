import { describe, expect, it } from "vitest";
import { emptyDefaultValueConverter } from "@deterministic-code/datasources";
import { MemoryDriver } from "./MemoryDriver.ts";

describe("MemoryDriver converters", () => {
  it("leaves type and row converters empty", () => {
    const driver = new MemoryDriver();
    expect(driver.typeConverter).toBeUndefined();
    expect(driver.rowTypeConverters).toEqual({});
    expect(driver.defaultValueConverter).toBe(emptyDefaultValueConverter);
    expect(driver.defaultValueConverter.convert("string", "hi")).toBeNull();
  });
});
