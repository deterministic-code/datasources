import { isAbsolute, join } from "node:path";
import { parseEnv, scheme, type EnvMap } from "../env.ts";
import type { Fs } from "../io.ts";
import { migrateProviders } from "../migrateInit.ts";
import type { Flags, GenerateCommand } from "../parse.ts";
import type { Deps } from "../run.ts";
import { loadSpec } from "./loadSpec.ts";

const at = (root: string, path: string): string => (isAbsolute(path) ? path : join(root, path));

const aliases: Record<string, string> = {
  sqlite3: "sqlite",
  postgresql: "postgres",
  pg: "postgres",
  mariadb: "mysql",
  mssql: "sqlserver",
  mssqlserver: "sqlserver",
  ora: "oracle",
};

const canonical = (name: string): string => {
  const key = name.trim().toLowerCase();
  return aliases[key] ?? key;
};

const isSql = (name: string): boolean => (migrateProviders as readonly string[]).includes(name);

const sqlDialects = (settings: Record<string, string>): string[] => {
  const out: string[] = [];
  for (const name of settings["backend.datasources"].split(",")) {
    const key = canonical(name);
    if (key !== "" && isSql(key) && !out.includes(key)) out.push(key);
  }
  return out;
};

const readEnv = async (fs: Fs, cwd: string): Promise<EnvMap> => {
  try {
    return parseEnv(await fs.readFile(join(cwd, ".env")));
  } catch {
    return {};
  }
};

const merged = (file: EnvMap, env: NodeJS.ProcessEnv): EnvMap => ({
  ...file,
  ...(env.PROVIDER ? { PROVIDER: env.PROVIDER } : {}),
  ...(env.DATASOURCE_URL ? { DATASOURCE_URL: env.DATASOURCE_URL } : {}),
});

const inMemory = (url: string): boolean => url === ":memory:" || url.includes(":memory:");

export const applyMigrateUp = async (
  command: GenerateCommand,
  flags: Flags,
  deps: Deps,
  env: NodeJS.ProcessEnv = process.env,
): Promise<void> => {
  const loaded = await loadSpec(deps.fs, at(flags.cwd, command.input));
  const dialects = sqlDialects(loaded.settings);
  const values = merged(await readEnv(deps.fs, flags.cwd), env);
  const connection = values.DATASOURCE_URL ?? "";
  const provider = canonical(values.PROVIDER || scheme(connection) || "");
  if (!provider || !connection) {
    deps.log("migrate-up skipped: set PROVIDER and DATASOURCE_URL, or pass --no-auto-migrate-up");
    return;
  }
  if (inMemory(connection)) {
    deps.log("migrate-up skipped: DATASOURCE_URL is in-memory");
    return;
  }
  if (!isSql(provider)) {
    deps.log(`migrate-up skipped: ${provider} has no SQL migrations`);
    return;
  }
  if (!dialects.includes(provider))
    throw new Error(
      `migrate-up: provider ${provider} is not in backend.datasources (${dialects.join(", ") || "none"})`,
    );
  const npx = (verb: string, extra: string[] = []) => [
    "--yes",
    "--package",
    "@deterministic-code/migraters",
    verb,
    "--provider",
    provider,
    "--connection",
    connection,
    ...extra,
  ];
  const setup = npx("migrate-setup");
  const up = npx("migrate-up", ["--migrations-root", command.migrationsRoot]);
  deps.log(`$ npx ${setup.join(" ")}`);
  if (!flags.dryRun) await deps.run("npx", setup, flags.cwd);
  deps.log(`$ npx ${up.join(" ")}`);
  if (!flags.dryRun) await deps.run("npx", up, flags.cwd);
};
