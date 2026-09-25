import { describe, expect, it } from "vitest";
import { DatasourceError } from "./index.ts";

describe("DatasourceError", () => {
  it("wraps a generic datasource error", () => {
    const error = new DatasourceError("broken");
    expect(error.name).toBe("DatasourceError");
    expect(error.message).toBe("broken");
  });
});
