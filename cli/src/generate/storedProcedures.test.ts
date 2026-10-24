import { describe, expect, it } from "vitest";
import { withStoredProcedures } from "./storedProcedures.ts";

describe("withStoredProcedures", () => {
  it("sets the key when the flag is on", () => {
    expect(withStoredProcedures({ "backend.datasources": "postgres" }, true)).toEqual({
      "backend.datasources": "postgres",
      "datasource.use_stored_procedures": "true",
    });
  });

  it("leaves a yaml true in place when the flag is off", () => {
    const settings = { "datasource.use_stored_procedures": "true" };
    expect(withStoredProcedures(settings, false)).toBe(settings);
  });

  it("leaves a missing key missing when the flag is off", () => {
    const settings = { "backend.datasources": "sqlite" };
    expect(withStoredProcedures(settings, false)).toBe(settings);
    expect(settings["datasource.use_stored_procedures"]).toBeUndefined();
  });
});
