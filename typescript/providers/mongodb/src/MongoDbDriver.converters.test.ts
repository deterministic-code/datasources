import { describe, expect, it } from "vitest";
import { emptyDefaultValueConverter } from "@deterministic-code/datasources";
import { MongoDbDriver } from "./MongoDbDriver.ts";

describe("MongoDbDriver converters", () => {
  it("leaves type and row converters empty", () => {
    const driver = new MongoDbDriver();
    expect(driver.typeConverter).toBeUndefined();
    expect(driver.rowTypeConverters).toEqual({});
    expect(driver.defaultValueConverter).toBe(emptyDefaultValueConverter);
  });
});
