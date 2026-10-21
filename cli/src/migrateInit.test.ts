import { describe, expect, it } from "vitest";
import { migrateInitCommands, migrateLanguage, sqlMigrateProviders } from "./migrateInit.ts";

describe("migrateInit", () => {
  it("keeps migraters dialects in catalog order and skips non-SQL", () => {
    expect(sqlMigrateProviders(["json", "postgres", "sqlite", "mongodb"])).toEqual(["sqlite", "postgres"]);
    expect(migrateLanguage("ts")).toBe("typescript");
    expect(migrateLanguage("csharp")).toBe("csharp");
    expect(migrateLanguage("rust")).toBe("rust");
    expect(migrateInitCommands([], "csharp")).toEqual([]);
    expect(migrateInitCommands(["sqlite", "postgres"], "csharp")).toEqual([
      { command: "npm", args: ["install", "@deterministic-code/migraters"] },
      {
        command: "npx",
        args: [
          "--yes",
          "--package",
          "@deterministic-code/migraters",
          "migrate-init",
          "--language",
          "csharp",
          "--dialect",
          "sqlite",
          "--dialect",
          "postgres",
        ],
      },
    ]);
  });
});
