import { describe, expect, it } from "vitest";
import { overlayEnv } from "../env.ts";
import { sqlDialectsFromEnv } from "./sqlDialects.ts";

describe("sqlDialectsFromEnv", () => {
  it("uses INSTALLED_PROVIDERS, not settings", () => {
    expect(sqlDialectsFromEnv({ INSTALLED_PROVIDERS: "mysql,postgres" })).toEqual(["mysql", "postgres"]);
    expect(sqlDialectsFromEnv({ INSTALLED_PROVIDERS: "mysql", PROVIDER: "sqlite" })).toEqual(["mysql"]);
    expect(sqlDialectsFromEnv({ PROVIDER: "PostgreSQL" })).toEqual(["postgres"]);
    expect(sqlDialectsFromEnv({ DATASOURCE_URL: "sqlite:///./app.sqlite" })).toEqual(["sqlite"]);
    expect(sqlDialectsFromEnv({ INSTALLED_PROVIDERS: "json,mysql" })).toEqual(["mysql"]);
    expect(sqlDialectsFromEnv({})).toEqual([]);
  });

  it("lets process env overlay the dest file", () => {
    expect(
      overlayEnv({ INSTALLED_PROVIDERS: "sqlite,mysql" }, { INSTALLED_PROVIDERS: "mysql" }),
    ).toEqual({ INSTALLED_PROVIDERS: "mysql" });
  });
});
