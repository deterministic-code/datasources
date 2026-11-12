import { describe, expect, it } from "vitest";
import { emptyDefaultValueConverter } from "@deterministic-code/datasources";
import { XmlDriver } from "./XmlDriver.ts";

describe("XmlDriver converters", () => {
  it("leaves type and row converters empty", () => {
    const driver = new XmlDriver();
    expect(driver.typeConverter).toBeUndefined();
    expect(driver.rowTypeConverters).toEqual({});
    expect(driver.defaultValueConverter).toBe(emptyDefaultValueConverter);
  });
});
