import { isAbsolute, join } from "node:path";
import { overlayEnv, parseEnv, scheme, type EnvMap } from "../env.ts";
import type { Fs } from "../io.ts";
import type { Flags, GenerateCommand } from "../parse.ts";
import type { Deps } from "../io.ts";
import { loadSpec } from "./loadSpec.ts";
import { canonical, isSql, sqlDialectsFromEnv } from "./sqlDialects.ts";

const at = (root: string, path: string): string => (isAbsolute(path) ? path : join(root, path));

const readEnv = async (fs: Fs, cwd: string): Promise<EnvMap> => {
  try {
    return parseEnv(await fs.readFile(join(cwd, ".env")));
  } catch {
    return {};
  }
};

const inMemory = (url: string): boolean => url === ":memory:" || url.includes(":memory:");

export const applyMigrateUp = async (
  command: GenerateCommand,
  flags: Flags,
  deps: Deps,
  env: NodeJS.ProcessEnv = process.env,
): Promise<void> => {
  await loadSpec(deps.fs, at(flags.cwd, command.input));
  const values = overlayEnv(await readEnv(deps.fs, flags.cwd), env);
  const dialects = sqlDialectsFromEnv(values);
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
      `migrate-up: provider ${provider} is not in INSTALLED_PROVIDERS (${dialects.join(", ") || "none"})`,
    );
  const npx = (verb: string, extra: string[] = []) => [
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
