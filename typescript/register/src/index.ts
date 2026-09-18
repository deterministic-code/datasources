import { install } from "./install.ts";

const envKeys = [
  "INSTALLED_PROVIDERS",
  "providers",
  "DATABASE_URL",
  "url",
  "DATABASE_SCHEMA",
  "provider",
] as const;

const env: Record<string, string> = {};
if (typeof process !== "undefined") {
  for (const key of envKeys) {
    const value = process.env[key];
    if (value?.trim()) env[key] = value;
  }
}
if (Object.keys(env).length) await install(env);

export { install, type ProviderLoader } from "./install.ts";
export { providerNames } from "./providerNames.ts";
