import { scheme, splitProviders, type EnvMap } from "../env.ts";
import { migrateProviders } from "../migrateInit.ts";

const aliases: Record<string, string> = {
  sqlite3: "sqlite",
  postgresql: "postgres",
  pg: "postgres",
  mariadb: "mysql",
  mssql: "sqlserver",
  mssqlserver: "sqlserver",
  ora: "oracle",
};

export const canonical = (name: string): string => {
  const key = name.trim().toLowerCase();
  return aliases[key] ?? key;
};

export const isSql = (name: string): boolean => (migrateProviders as readonly string[]).includes(name);

export const sqlDialectsFromEnv = (env: EnvMap): string[] => {
  const out: string[] = [];
  for (const name of splitProviders(env.INSTALLED_PROVIDERS || env.PROVIDER || scheme(env.DATASOURCE_URL || "") || "")) {
    const key = canonical(name);
    if (key !== "" && isSql(key) && !out.includes(key)) out.push(key);
  }
  return out;
};
