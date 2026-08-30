import { describe, expect, it } from "vitest";
import {
  answersFromEnv,
  ensureProviders,
  parseEnv,
  serializeEnv,
  splitProviders,
  updateEnvProviders,
} from "./env.ts";

describe("env", () => {
  it("parses, skips comments, and serializes known keys first", () => {
    const env = parseEnv("# hi\n\nDATASOURCE_PROVIDER=sqlite\nEXTRA=1\nbroken\n");
    expect(env).toEqual({ DATASOURCE_PROVIDER: "sqlite", EXTRA: "1" });
    expect(serializeEnv({ EXTRA: "1", DATASOURCE_PROVIDER: "json" })).toBe(
      "DATASOURCE_PROVIDER=json\nEXTRA=1\n",
    );
  });

  it("keeps the default provider in the list", () => {
    expect(splitProviders("sqlite, json, sqlite")).toEqual(["sqlite", "json"]);
    expect(ensureProviders(["json"], "sqlite")).toEqual(["json", "sqlite"]);
    expect(ensureProviders(["sqlite"], "sqlite")).toEqual(["sqlite"]);
  });

  it("defaults answers from env or sqlite", () => {
    expect(answersFromEnv({})).toEqual({
      provider: "sqlite",
      providers: ["sqlite"],
      filename: ":memory:",
      url: "",
      schema: "",
      engine: true,
    });
    expect(
      answersFromEnv({
        DATASOURCE_PROVIDER: "mysql",
        DATASOURCE_PROVIDERS: "mysql,json",
        SQLITE_FILENAME: "app.db",
        DATABASE_URL: "mysql://x",
        DATABASE_SCHEMA: "public",
      }).providers,
    ).toEqual(["mysql", "json"]);
  });

  it("updates provider lists on add and remove", () => {
    expect(
      updateEnvProviders(
        { DATASOURCE_PROVIDER: "sqlite", DATASOURCE_PROVIDERS: "sqlite" },
        ["json"],
        [],
      ),
    ).toEqual({
      DATASOURCE_PROVIDER: "sqlite",
      DATASOURCE_PROVIDERS: "sqlite,json",
    });
    expect(
      updateEnvProviders(
        { DATASOURCE_PROVIDER: "sqlite", DATASOURCE_PROVIDERS: "sqlite,json" },
        [],
        ["sqlite"],
      ),
    ).toEqual({ DATASOURCE_PROVIDER: "json", DATASOURCE_PROVIDERS: "json" });
    expect(updateEnvProviders({}, ["mysql"], [])).toEqual({
      DATASOURCE_PROVIDER: "mysql",
      DATASOURCE_PROVIDERS: "mysql",
    });
    expect(updateEnvProviders({ DATASOURCE_PROVIDER: "sqlite" }, [], ["sqlite"])).toEqual({
      DATASOURCE_PROVIDER: "sqlite",
      DATASOURCE_PROVIDERS: "sqlite",
    });
  });
});
