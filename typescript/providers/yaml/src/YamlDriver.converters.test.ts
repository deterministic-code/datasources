import { describe, expect, it } from "vitest";
import { emptyDefaultValueConverter } from "@deterministic-code/datasources";
import { YamlDriver } from "./YamlDriver.ts";

describe("YamlDriver converters", () => {
  it("leaves type and row converters empty", () => {
    const driver = new YamlDriver();
    expect(driver.typeConverter).toBeUndefined();
    expect(driver.rowTypeConverters).toEqual({});
    expect(driver.defaultValueConverter).toBe(emptyDefaultValueConverter);
  });
});
