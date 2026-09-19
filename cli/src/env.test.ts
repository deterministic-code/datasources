import { describe, expect, it } from "vitest";
import {
  answersFromEnv,
  ensureProviders,
  parseEnv,
  serializeEnv,
  splitProviders,
  updateEnvProviders,
} from "./env.ts";
import { providerFromUrl } from "./providerFromUrl.ts";

describe("env", () => {
  it("parses, skips comments, and serializes known keys first", () => {
    const env = parseEnv("# hi\n\nINSTALLED_PROVIDERS=sqlite\nEXTRA=1\nbroken\n");
    expect(env).toEqual({ INSTALLED_PROVIDERS: "sqlite", EXTRA: "1" });
    expect(serializeEnv({ EXTRA: "1", INSTALLED_PROVIDERS: "json" })).toBe(
      "INSTALLED_PROVIDERS=json\nEXTRA=1\n",
    );
  });

  it("keeps the default provider in the list", () => {
    expect(splitProviders("sqlite, json, sqlite")).toEqual(["sqlite", "json"]);
    expect(ensureProviders(["json"], "sqlite")).toEqual(["json", "sqlite"]);
    expect(ensureProviders(["sqlite"], "sqlite")).toEqual(["sqlite"]);
  });

  it("defaults answers from env or sqlite url", () => {
    expect(answersFromEnv({})).toEqual({
      providers: ["sqlite"],
      url: "sqlite:///:memory:",
      schema: "",
    });
    expect(
      answersFromEnv({
        INSTALLED_PROVIDERS: "mysql,json",
        DATASOURCE_URL: "mysql://x",
        DATABASE_SCHEMA: "public",
      }),
    ).toEqual({
      providers: ["mysql", "json"],
      url: "mysql://x",
      schema: "public",
    });
    expect(answersFromEnv({ DATASOURCE_PROVIDER: "postgres" }).providers).toEqual(["postgres"]);
    expect(answersFromEnv({ INSTALLED_PROVIDERS: "mysql" }).url).toBe("");
    expect(answersFromEnv({ INSTALLED_PROVIDERS: "sqlite" }).url).toBe("sqlite:///:memory:");
    expect(answersFromEnv({ DATASOURCE_URL: "not-a-url" }).providers).toEqual(["sqlite"]);
    expect(answersFromEnv({ DATABASE_URL: "mysql://legacy" }).url).toBe("mysql://legacy");
    expect(answersFromEnv({ SQLITE_FILENAME: "app.db" }).url).toBe("sqlite:///app.db");
    expect(answersFromEnv({ SQLITE_FILENAME: ":memory:" }).url).toBe("sqlite:///:memory:");
    expect(answersFromEnv({ SQLITE_FILENAME: ":memory" }).url).toBe("sqlite:///:memory:");
    expect(providerFromUrl("postgresql://localhost/db")).toBe("postgres");
    expect(providerFromUrl("sqlite:///:memory:")).toBe("sqlite");
  });

  it("updates provider lists on add and remove", () => {
    expect(
      updateEnvProviders({ INSTALLED_PROVIDERS: "sqlite" }, ["json"], []),
    ).toEqual({ INSTALLED_PROVIDERS: "sqlite,json" });
    expect(
      updateEnvProviders({ INSTALLED_PROVIDERS: "sqlite,json" }, [], ["sqlite"]),
    ).toEqual({ INSTALLED_PROVIDERS: "json" });
    expect(updateEnvProviders({}, ["mysql"], [])).toEqual({ INSTALLED_PROVIDERS: "mysql" });
    expect(updateEnvProviders({ DATASOURCE_PROVIDER: "sqlite" }, [], ["sqlite"])).toEqual({
      INSTALLED_PROVIDERS: "sqlite",
    });
    expect(
      updateEnvProviders({ DATABASE_URL: "mysql://x" }, ["json"], []),
    ).toEqual({
      DATASOURCE_URL: "mysql://x",
      INSTALLED_PROVIDERS: "mysql,json",
    });
  });
});
