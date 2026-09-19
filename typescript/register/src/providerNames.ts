import { providerFromUrl, type DatasourceOptions } from "@datasources/core";

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
  const names = [...new Set(split(text(options, "INSTALLED_PROVIDERS")))];
  const provider = providerFromUrl(text(options, "DATASOURCE_URL") ?? "");
  return provider && !names.includes(provider) ? [...names, provider] : names;
};
