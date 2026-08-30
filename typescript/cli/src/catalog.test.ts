import { describe, expect, it } from "vitest";
import { catalog, isProvider, providers, requireProviders } from "./catalog.ts";

describe("catalog", () => {
  it("lists every dialect with npm, cargo, and nuget ids", () => {
    expect(providers).toHaveLength(12);
    for (const name of providers) {
      expect(catalog[name].npm).toBe(`@datasources/${name}`);
      expect(catalog[name].cargo).toMatch(/^datasources-/);
      expect(catalog[name].nuget).toMatch(/^Datasources\./);
    }
    expect(catalog.sqlite.npmEngine).toBe("better-sqlite3");
    expect(catalog.sqlite.cargoFeatures).toEqual(["rusqlite"]);
    expect(catalog.sqlite.nugetEngine).toBe("Datasources.Sqlite.Microsoft");
    expect(catalog.mysql.npmEngine).toBe("mysql2");
    expect(catalog.postgres.npmEngine).toBe("pg");
  });

  it("validates provider names", () => {
    expect(isProvider("sqlite")).toBe(true);
    expect(isProvider("nope")).toBe(false);
    expect(requireProviders(["json", "memory"])).toEqual(["json", "memory"]);
    expect(() => requireProviders(["nope"])).toThrow(/Unknown provider: nope/);
  });
});
