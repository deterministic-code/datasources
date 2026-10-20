import { describe, expect, it } from "vitest";
import { migrateInitCommands, sqlMigrateProviders } from "./migrateInit.ts";

describe("migrateInit", () => {
  it("keeps migraters dialects in catalog order and skips non-SQL", () => {
    expect(sqlMigrateProviders(["json", "postgres", "sqlite", "mongodb"])).toEqual(["sqlite", "postgres"]);
    expect(migrateInitCommands([])).toEqual([]);
    expect(migrateInitCommands(["sqlite", "postgres"])).toEqual([
      { command: "npm", args: ["install", "@deterministic-code/migraters"] },
      {
        command: "npx",
        args: [
          "--yes",
          "--package",
          "@deterministic-code/migraters",
          "migrate-init",
          "--dialect",
          "sqlite",
          "--dialect",
          "postgres",
        ],
      },
    ]);
  });
});
