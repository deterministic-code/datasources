import { describe, it } from "vitest";
import { assertCells, assertDefaults, assertTypes } from "../../loadDriverConversions.ts";
import { SqliteDriver } from "./SqliteDriver.ts";
import { SqliteRowTypeConverters } from "./converters/SqliteRowTypeConverters.ts";

describe("sqlite conversion matrix", () => {
  it("asserts every documented conversion", () => {
    const driver = new SqliteDriver();
    assertTypes("sqlite", driver.typeConverter);
    assertDefaults("sqlite", driver.defaultValueConverter);
    assertCells("sqlite", SqliteRowTypeConverters);
  });
});
