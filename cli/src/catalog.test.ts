import { describe, expect, it } from "vitest";
import { catalog, isProvider, providers, requireProviders } from "./catalog.ts";

describe("catalog", () => {
  it("lists every dialect with npm, cargo, and nuget ids", () => {
    expect(providers).toHaveLength(11);
    for (const name of providers) {
      expect(catalog[name].npm).toBe(`@deterministic-code/datasources-${name}`);
      expect(catalog[name].cargo).toMatch(/^datasources-/);
      expect(catalog[name].nuget).toMatch(/^Datasources\./);
    }
    expect(catalog.sqlite.nuget).toBe("Datasources.Sqlite");
  });

  it("validates provider names", () => {
    expect(isProvider("sqlite")).toBe(true);
    expect(isProvider("nope")).toBe(false);
    expect(requireProviders(["json", "memory"])).toEqual(["json", "memory"]);
    expect(() => requireProviders(["nope"])).toThrow(/Unknown provider: nope/);
  });
});
