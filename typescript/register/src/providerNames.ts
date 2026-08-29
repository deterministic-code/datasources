import type { DatasourceOptions } from "@datasources/core";

const text = (options: DatasourceOptions, key: string): string | undefined => {
  const value = options[key];
  return typeof value === "string" && value.trim() ? value.trim() : undefined;
};

const split = (value: string | undefined): string[] =>
  (value ?? "")
    .split(",")
    .map((name) => name.trim())
    .filter((name) => name.length > 0);

export const providerNames = (options: DatasourceOptions): string[] => {
  const names = [...new Set(split(text(options, "providers") ?? text(options, "DATASOURCE_PROVIDERS")))];
  const provider = text(options, "provider") ?? text(options, "DATASOURCE_PROVIDER");
  return provider && !names.includes(provider) ? [...names, provider] : names;
};
