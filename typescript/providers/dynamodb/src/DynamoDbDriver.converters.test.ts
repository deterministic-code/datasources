import { describe, expect, it } from "vitest";
import { emptyDefaultValueConverter } from "@deterministic-code/datasources";
import { DynamoDbDriver } from "./DynamoDbDriver.ts";

describe("DynamoDbDriver converters", () => {
  it("leaves type and row converters empty", () => {
    const driver = new DynamoDbDriver();
    expect(driver.typeConverter).toBeUndefined();
    expect(driver.rowTypeConverters).toEqual({});
    expect(driver.defaultValueConverter).toBe(emptyDefaultValueConverter);
  });
});
