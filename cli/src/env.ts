import { providerFromUrl } from "./providerFromUrl.ts";

export type EnvMap = Record<string, string>;

export const knownEnvKeys = ["INSTALLED_PROVIDERS", "DATASOURCE_URL", "DATABASE_SCHEMA"] as const;

const droppedEnvKeys = [
  "DATABASE_URL",
  "DEFAULT_PROVIDER",
  "DATASOURCE_PROVIDER",
  "DATASOURCE_PROVIDERS",
  "SQLITE_FILENAME",
] as const;

const urlOf = (env: EnvMap) => env.DATASOURCE_URL || env.DATABASE_URL || "";

const installedOf = (env: EnvMap) =>
  env.INSTALLED_PROVIDERS ||
  env.DATASOURCE_PROVIDERS ||
  providerFromUrl(urlOf(env)) ||
  env.DEFAULT_PROVIDER ||
  env.DATASOURCE_PROVIDER ||
  "";

export const parseEnv = (text: string): EnvMap =>
  Object.fromEntries(
    text
      .split(/\r?\n/)
      .filter((line) => line.trim() && !line.trimStart().startsWith("#") && line.includes("="))
      .map((line) => {
        const i = line.indexOf("=");
        return [line.slice(0, i).trim(), line.slice(i + 1)] as const;
      }),
  );

export const serializeEnv = (map: EnvMap): string => {
  const extra = Object.keys(map).filter(
    (key) =>
      !(knownEnvKeys as readonly string[]).includes(key) &&
      !(droppedEnvKeys as readonly string[]).includes(key),
  );
  return [...knownEnvKeys.filter((key) => key in map), ...extra]
    .map((key) => `${key}=${map[key]}`)
    .join("\n")
    .concat("\n");
};

export const splitProviders = (value: string): string[] => [
  ...new Set(
    value
      .split(",")
      .map((name) => name.trim())
      .filter(Boolean),
  ),
];

export const ensureProviders = (names: string[], defaultProvider: string): string[] =>
  names.includes(defaultProvider) ? names : [...names, defaultProvider];

export const updateEnvProviders = (
  env: EnvMap,
  add: string[],
  remove: string[],
): EnvMap => {
  const current = splitProviders(installedOf(env));
  const next = [
    ...current.filter((name) => !remove.includes(name)),
    ...add.filter((name) => !current.includes(name)),
  ];
  const fallback = providerFromUrl(urlOf(env)) ?? next[0] ?? "sqlite";
  const {
    DATABASE_URL: _u,
    DEFAULT_PROVIDER: _d,
    DATASOURCE_PROVIDER: _p,
    DATASOURCE_PROVIDERS: _ps,
    ...rest
  } = env;
  return {
    ...rest,
    ...(urlOf(env) ? { DATASOURCE_URL: urlOf(env) } : {}),
    INSTALLED_PROVIDERS: (next.length ? next : [fallback]).join(","),
  };
};

export const sqliteUrl = (filename: string): string =>
  filename === ":memory:" || filename === ":memory" ? "sqlite:///:memory:" : `sqlite:///${filename}`;

export const answersFromEnv = (env: EnvMap) => {
  const url =
    urlOf(env) ||
    (env.SQLITE_FILENAME ? sqliteUrl(env.SQLITE_FILENAME) : "") ||
    (installedOf(env).includes("sqlite") || !installedOf(env) ? "sqlite:///:memory:" : "");
  return {
    providers: splitProviders(installedOf(env) || providerFromUrl(url) || "sqlite"),
    url,
    schema: env.DATABASE_SCHEMA ?? "",
  };
};
