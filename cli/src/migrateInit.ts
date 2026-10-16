import type { Provider } from "./catalog.ts";
import type { Proc } from "./commands.ts";

export const migrateProviders = ["sqlite", "postgres", "mysql", "sqlserver", "oracle"] as const;

export const sqlMigrateProviders = (names: readonly Provider[]): Provider[] =>
  migrateProviders.filter((name) => names.includes(name));

export const migrateInitCommands = (dialects: Provider[]): Proc[] =>
  dialects.length === 0
    ? []
    : [
        { command: "npm", args: ["install", "@deterministic-code/migraters"] },
        { command: "npx", args: ["migrate-init", ...dialects.flatMap((name) => ["--dialect", name])] },
      ];
