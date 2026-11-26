import { describe, it } from "vitest";
import { assertCells, assertDefaults, assertTypes } from "../../loadDriverConversions.ts";
import { PostgresDriver } from "./PostgresDriver.ts";
import { PostgresRowTypeConverters } from "./converters/PostgresRowTypeConverters.ts";

describe("postgres conversion matrix", () => {
  it("asserts every documented conversion", () => {
    const driver = new PostgresDriver();
    assertTypes("postgres", driver.typeConverter);
    assertDefaults("postgres", driver.defaultValueConverter);
    assertCells("postgres", PostgresRowTypeConverters);
  });
});
