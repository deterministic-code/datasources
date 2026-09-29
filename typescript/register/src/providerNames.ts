import { DatasourceError, type DatasourceOptions } from "@datasources/core";

const scheme = (url: string): string | undefined =>
  url.match(/^([a-zA-Z][a-zA-Z0-9+.-]*):/)?.[1]?.toLowerCase();

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
  const url = text(options, "DATASOURCE_URL");
  if (!url) return names;
  const provider = scheme(url);
  if (!provider) throw new DatasourceError("DATASOURCE_URL has no provider");
  return names.includes(provider) ? names : [...names, provider];
};
