import { describe, expect, it } from "vitest";
import {
  answersFromEnv,
  ensureProviders,
  parseEnv,
  schemaOf,
  serializeEnv,
  splitProviders,
  updateEnvProviders,
  withSchema,
} from "./env.ts";

describe("env", () => {
  it("parses, skips comments, and serializes known keys first", () => {
    const env = parseEnv("# hi\n\nINSTALLED_PROVIDERS=sqlite\nEXTRA=1\nbroken\n");
    expect(env).toEqual({ INSTALLED_PROVIDERS: "sqlite", EXTRA: "1" });
    expect(serializeEnv({ EXTRA: "1", INSTALLED_PROVIDERS: "json", SQLITE_FILENAME: "app.db" })).toBe(
      "INSTALLED_PROVIDERS=json\nEXTRA=1\n",
    );
    expect(schemaOf("postgres://db/app?&schema")).toBe("");
    expect(schemaOf("postgres://db/app?schema=public")).toBe("public");
    expect(withSchema("postgres://db/app#frag", "public")).toBe(
      "postgres://db/app?schema=public#frag",
    );
    expect(withSchema("postgres://db/app?x=1", "public")).toBe(
      "postgres://db/app?x=1&schema=public",
    );
    expect(withSchema("postgres://db/app?schema=app", "public")).toBe("postgres://db/app?schema=app");
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
        DATASOURCE_URL: "mysql://x?schema=public",
      }),
    ).toEqual({
      providers: ["mysql", "json"],
      url: "mysql://x?schema=public",
      schema: "public",
    });
    expect(answersFromEnv({ DATASOURCE_PROVIDER: "postgres" })).toEqual({
      providers: ["sqlite"],
      url: "sqlite:///:memory:",
      schema: "",
    });
    expect(answersFromEnv({ INSTALLED_PROVIDERS: "mysql" }).url).toBe("");
    expect(answersFromEnv({ INSTALLED_PROVIDERS: "sqlite" }).url).toBe("sqlite:///:memory:");
    expect(answersFromEnv({ DATASOURCE_URL: "not-a-url" }).providers).toEqual(["sqlite"]);
    expect(answersFromEnv({ DATABASE_URL: "mysql://legacy" })).toEqual({
      providers: ["sqlite"],
      url: "sqlite:///:memory:",
      schema: "",
    });
    expect(answersFromEnv({ DATASOURCE_URL: "postgresql://localhost/db" }).providers).toEqual([
      "postgresql",
    ]);
    expect(answersFromEnv({ DATASOURCE_URL: "postgres://localhost/db" }).providers).toEqual([
      "postgres",
    ]);
    expect(answersFromEnv({ DATASOURCE_URL: "sqlite:///:memory:" }).providers).toEqual(["sqlite"]);
  });

  it("updates provider lists on add and remove", () => {
    expect(
      updateEnvProviders({ INSTALLED_PROVIDERS: "sqlite" }, ["json"], []),
    ).toEqual({ INSTALLED_PROVIDERS: "sqlite,json" });
    expect(
      updateEnvProviders({ INSTALLED_PROVIDERS: "sqlite,json" }, [], ["sqlite"]),
    ).toEqual({ INSTALLED_PROVIDERS: "json" });
    expect(updateEnvProviders({}, ["mysql"], [])).toEqual({ INSTALLED_PROVIDERS: "mysql" });
    expect(updateEnvProviders({}, [], [])).toEqual({ INSTALLED_PROVIDERS: "sqlite" });
    expect(
      updateEnvProviders(
        {
          DATASOURCE_PROVIDER: "postgres",
          DATABASE_URL: "mysql://x",
          DATABASE_SCHEMA: "public",
          SQLITE_FILENAME: "app.db",
          DEFAULT_PROVIDER: "sqlite",
          DEFAULT_DATASOURCE_URL: "sqlite:///:memory:",
        },
        ["json"],
        [],
      ),
    ).toEqual({ INSTALLED_PROVIDERS: "json" });
    expect(
      updateEnvProviders(
        {
          DATASOURCE_URL: "postgres://localhost/db",
          DATABASE_SCHEMA: "public",
          INSTALLED_PROVIDERS: "postgres",
        },
        [],
        [],
      ),
    ).toEqual({
      DATASOURCE_URL: "postgres://localhost/db",
      INSTALLED_PROVIDERS: "postgres",
    });
  });
});
