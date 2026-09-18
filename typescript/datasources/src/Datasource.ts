import { createRequire } from "node:module";
import type { IDatasourceFactory } from "./IDatasourceFactory.ts";
import type { DatasourceOptions } from "./DatasourceOptions.ts";
import type { IDatasource } from "./IDatasource.ts";
import { DatasourceError } from "./DatasourceError.ts";
import { ProviderNotInstalledError } from "./ProviderNotInstalledError.ts";
import { providerFromUrl } from "./providerFromUrl.ts";

const registry = new Map<string, IDatasourceFactory>();
let specified: DatasourceOptions | undefined;
let installListed: ((options: DatasourceOptions) => void) | undefined;

const envKeys = [
  "INSTALLED_PROVIDERS",
  "providers",
  "DATABASE_URL",
  "url",
  "DATABASE_SCHEMA",
  "provider",
] as const;

const text = (options: DatasourceOptions | undefined, key: string): string | undefined => {
  const value = options?.[key];
  return typeof value === "string" && value.trim() ? value.trim() : undefined;
};

const providerName = (options?: DatasourceOptions) =>
  text(options, "provider") ?? providerFromUrl(text(options, "url") ?? text(options, "DATABASE_URL") ?? "");

const fromEnv = (): DatasourceOptions => {
  const options: DatasourceOptions = {};
  for (const key of envKeys) {
    const value = process.env[key];
    if (value?.trim()) options[key] = value;
  }
  return options;
};

const bindRegister = (): void => {
  if (installListed) return;
  try {
    createRequire(import.meta.url)("@datasources/register");
  } catch {
    /* register package is not installed */
  }
};

export class Datasource {
  static bindInstall = (fn: (options: DatasourceOptions) => void): void => {
    installListed = fn;
  };

  static defaults = (options: DatasourceOptions): void => {
    bindRegister();
    installListed?.(options);
    specified = { ...options };
  };

  static register(...factories: IDatasourceFactory[]): void {
    for (const factory of factories) registry.set(factory.provider, factory);
  }

  static create(provider?: string, options?: DatasourceOptions): IDatasource {
    bindRegister();
    const listed = options ?? specified ?? fromEnv();
    if (!specified && !options && Object.keys(listed).length > 0) Datasource.defaults(listed);
    else installListed?.(listed);
    const merged = { ...specified, ...options };
    const name = provider ?? providerName(merged);
    if (!name) throw new DatasourceError("Datasource defaults are not specified");
    const factory = registry.get(name);
    if (!factory) throw new ProviderNotInstalledError(name);
    return factory.create(merged);
  }

  static reset(): void {
    registry.clear();
    specified = undefined;
  }

  static registered = (): string[] => [...registry.keys()];
}
