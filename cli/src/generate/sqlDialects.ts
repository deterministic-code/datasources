import { scheme, splitProviders, type EnvMap } from "../env.ts";
import { migrateProviders } from "../migrateInit.ts";

export const isSql = (name: string): boolean => (migrateProviders as readonly string[]).includes(name);

export const sqlDialectsFromEnv = (env: EnvMap): string[] => {
  const out: string[] = [];
  for (const name of splitProviders(env.INSTALLED_PROVIDERS || env.PROVIDER || scheme(env.DATASOURCE_URL || "") || "")) {
    if (name !== "" && isSql(name) && !out.includes(name)) out.push(name);
  }
  return out;
};
