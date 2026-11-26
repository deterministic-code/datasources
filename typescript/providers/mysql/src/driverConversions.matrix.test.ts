import { describe, it } from "vitest";
import { assertCells, assertDefaults, assertTypes } from "../../loadDriverConversions.ts";
import { MysqlDriver } from "./MysqlDriver.ts";
import { MysqlRowTypeConverters } from "./converters/MysqlRowTypeConverters.ts";

describe("mysql conversion matrix", () => {
  it("asserts every documented conversion", () => {
    const driver = new MysqlDriver();
    assertTypes("mysql", driver.typeConverter);
    assertDefaults("mysql", driver.defaultValueConverter);
    assertCells("mysql", MysqlRowTypeConverters);
  });
});
