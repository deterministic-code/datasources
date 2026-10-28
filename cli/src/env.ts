export type EnvMap = Record<string, string>;

export const scheme = (url: string): string | undefined =>
  url.match(/^([a-zA-Z][a-zA-Z0-9+.-]*):/)?.[1]?.toLowerCase();

export const knownEnvKeys = ["INSTALLED_PROVIDERS", "PROVIDER", "DATASOURCE_URL"] as const;

const droppedEnvKeys = [
  "DATABASE_URL",
  "DATABASE_SCHEMA",
  "DEFAULT_PROVIDER",
  "DEFAULT_DATASOURCE_URL",
  "DATASOURCE_PROVIDER",
  "DATASOURCE_PROVIDERS",
  "SQLITE_FILENAME",
] as const;

export const schemaOf = (url: string): string => {
  const query = url.split("#")[0]?.split("?")[1] ?? "";
  for (const part of query.split("&")) {
    if (!part) continue;
    const eq = part.indexOf("=");
    const key = decodeURIComponent(eq < 0 ? part : part.slice(0, eq));
    if (key !== "schema") continue;
    return decodeURIComponent(eq < 0 ? "" : part.slice(eq + 1));
  }
  return "";
};

export const withSchema = (url: string, schema: string): string => {
  if (!url || !schema || schemaOf(url)) return url;
  const hash = url.indexOf("#");
  const base = hash < 0 ? url : url.slice(0, hash);
  const fragment = hash < 0 ? "" : url.slice(hash);
  return `${base}${base.includes("?") ? "&" : "?"}schema=${encodeURIComponent(schema)}${fragment}`;
};

const urlOf = (env: EnvMap) => env.DATASOURCE_URL || "";

const installedOf = (env: EnvMap) => env.INSTALLED_PROVIDERS || scheme(urlOf(env)) || "";

export const overlayEnv = (file: EnvMap, env: NodeJS.ProcessEnv): EnvMap => ({
  ...file,
  ...(env.INSTALLED_PROVIDERS ? { INSTALLED_PROVIDERS: env.INSTALLED_PROVIDERS } : {}),
  ...(env.PROVIDER ? { PROVIDER: env.PROVIDER } : {}),
  ...(env.DATASOURCE_URL ? { DATASOURCE_URL: env.DATASOURCE_URL } : {}),
});

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
  const fallback = scheme(urlOf(env)) ?? next[0] ?? "sqlite";
  const {
    DATABASE_URL: _u,
    DATABASE_SCHEMA: _s,
    DEFAULT_PROVIDER: _d,
    DEFAULT_DATASOURCE_URL: _du,
    DATASOURCE_PROVIDER: _p,
    DATASOURCE_PROVIDERS: _ps,
    SQLITE_FILENAME: _f,
    ...rest
  } = env;
  const url = urlOf(env);
  return {
    ...rest,
    ...(url ? { DATASOURCE_URL: url } : {}),
    INSTALLED_PROVIDERS: (next.length ? next : [fallback]).join(","),
  };
};

export const answersFromEnv = (env: EnvMap) => {
  const url =
    urlOf(env) ||
    (installedOf(env).includes("sqlite") || !installedOf(env) ? "sqlite:///:memory:" : "");
  return {
    providers: splitProviders(installedOf(env) || scheme(url) || "sqlite"),
    url,
    schema: schemaOf(url),
  };
};
