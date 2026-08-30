export type EnvMap = Record<string, string>;

export const knownEnvKeys = [
  "DATASOURCE_PROVIDER",
  "DATASOURCE_PROVIDERS",
  "SQLITE_FILENAME",
  "DATABASE_URL",
  "DATABASE_SCHEMA",
] as const;

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
    (key) => !(knownEnvKeys as readonly string[]).includes(key),
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
  const current = splitProviders(env.DATASOURCE_PROVIDERS ?? env.DATASOURCE_PROVIDER ?? "");
  const next = ensureProviders(
    [...current.filter((name) => !remove.includes(name)), ...add.filter((name) => !current.includes(name))],
    env.DATASOURCE_PROVIDER && !remove.includes(env.DATASOURCE_PROVIDER)
      ? env.DATASOURCE_PROVIDER
      : add[0] ?? current.find((name) => !remove.includes(name)) ?? "sqlite",
  );
  const provider =
    env.DATASOURCE_PROVIDER && !remove.includes(env.DATASOURCE_PROVIDER)
      ? env.DATASOURCE_PROVIDER
      : next[0]!;
  return { ...env, DATASOURCE_PROVIDER: provider, DATASOURCE_PROVIDERS: next.join(",") };
};

export const answersFromEnv = (env: EnvMap) => ({
  provider: env.DATASOURCE_PROVIDER || "sqlite",
  providers: splitProviders(env.DATASOURCE_PROVIDERS || env.DATASOURCE_PROVIDER || "sqlite"),
  filename: env.SQLITE_FILENAME ?? ":memory:",
  url: env.DATABASE_URL ?? "",
  schema: env.DATABASE_SCHEMA ?? "",
  engine: true,
});
