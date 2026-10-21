import type { Ecosystem, Provider } from "./catalog.ts";
import type { Proc } from "./commands.ts";

export const migrateProviders = ["sqlite", "postgres", "mysql", "sqlserver", "oracle"] as const;

export type MigrateLanguage = "typescript" | "csharp" | "rust";

export const migrateLanguage = (ecosystem: Ecosystem): MigrateLanguage =>
  ecosystem === "ts" ? "typescript" : ecosystem;

export const sqlMigrateProviders = (names: readonly Provider[]): Provider[] =>
  migrateProviders.filter((name) => names.includes(name));

export const migrateInitCommands = (dialects: Provider[], language: MigrateLanguage): Proc[] =>
  dialects.length === 0
    ? []
    : [
        { command: "npm", args: ["install", "@deterministic-code/migraters"] },
        {
          command: "npx",
          args: [
            "--yes",
            "--package",
            "@deterministic-code/migraters",
            "migrate-init",
            "--language",
            language,
            ...dialects.flatMap((name) => ["--dialect", name]),
          ],
        },
      ];
