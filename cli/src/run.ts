import { access, copyFile, mkdir } from "node:fs/promises";
import { join } from "node:path";
import { catalog, requireProviders, type Ecosystem, type Provider } from "./catalog.ts";
import { addCommands, removeCommands, type Proc } from "./commands.ts";
import { csharpTarget, detect, resolveEcosystem } from "./detect.ts";
import { parseEnv, serializeEnv, updateEnvProviders, type EnvMap } from "./env.ts";
import { defaultDatasourceYaml } from "./generate/loadSpec.ts";
import type { Fs, Runner } from "./io.ts";
import {
  cargoProviders,
  csharpProviders,
  npmProviders,
  type NpmManifest,
} from "./installed.ts";
import { applyGenerate } from "./generate/applyGenerate.ts";
import { applyMigrations } from "./generate/applyMigrations.ts";
import { applyMigrateUp } from "./generate/migrateUp.ts";
import { migrateInitCommands, migrateLanguage, sqlMigrateProviders } from "./migrateInit.ts";
import { help, parseArgs, type Command, type Flags } from "./parse.ts";
import { applyVersion } from "./version/applyVersion.ts";
import { answersForSetup, envFromAnswers, promptSetup, starterTypesYaml, type Prompter, type SetupAnswers } from "./setup.ts";
import { wireRegister } from "./wireRegister.ts";
import type { Dialect } from "./generate/GenerateRequest.ts";

export type Deps = {
  fs: Fs;
  run: Runner;
  log: (line: string) => void;
  err: (line: string) => void;
  prompt: Prompter;
};

export const run = async (argv: string[], deps: Deps, cwd = "."): Promise<number> => {
  try {
    const parsed = parseArgs(argv, cwd);
    await dispatch(parsed.command, parsed.flags, deps);
    return 0;
  } catch (error) {
    deps.err(error instanceof Error ? error.message : "Unknown error");
    return 1;
  }
};

const dispatch = async (command: Command, flags: Flags, deps: Deps): Promise<void> => {
  if (command.kind === "help") return deps.log(help.trimEnd());
  if (command.kind === "setup") return applySetup(flags, deps, command.providers);
  if (command.kind === "add") return applyAddRemove(flags, deps, requireProviders(needNames(command.providers, "add")), []);
  if (command.kind === "remove") return applyAddRemove(flags, deps, [], requireProviders(needNames(command.providers, "remove")));
  if (command.kind === "version") return applyVersion(command, flags, deps);
  if (command.kind === "migrations-create") return applyMigrations(command, flags, deps);
  await applyGenerate(command, flags, deps);
  if (command.target !== "all") return;
  if (command.migrations)
    await applyMigrations(
      {
        kind: "migrations-create",
        provider: command.provider,
        input: command.input,
        storedProcedures: command.storedProcedures,
        migrationsRoot: command.migrationsRoot,
        ...(command.migrationsPath === undefined ? {} : { migrationsPath: command.migrationsPath }),
      },
      flags,
      deps,
    );
  if (command.migrate) await applyMigrateUp(command, flags, deps);
};

const needNames = (names: string[], verb: string): string[] =>
  names.length
    ? names
    : (() => {
        throw new Error(`Usage: datasources ${verb} <provider...>`);
      })();

const applySetup = async (flags: Flags, deps: Deps, requested: string[]): Promise<void> => {
  const entries = await deps.fs.readdir(flags.cwd);
  const found = detect(entries);
  const existing = await readEnv(deps.fs, flags.cwd);
  const specDir = join(flags.cwd, "deterministic");
  const typesPath = join(specDir, "types.yaml");
  const datasourcePath = join(specDir, "datasource.yaml");
  const missingSpec = (await optional(deps.fs, typesPath)) === undefined;
  const answers: SetupAnswers = flags.yes
    ? { ...answersForSetup(existing, requested), ecosystem: flags.ecosystem, scaffold: false }
    : await promptSetup(deps.prompt, existing, found, flags.ecosystem, missingSpec, requested);
  const ecosystem = resolveEcosystem(found, answers.ecosystem ?? flags.ecosystem);
  const selected = requireProviders(answers.providers);
  const installed = await readInstalled(deps.fs, flags.cwd, ecosystem, entries);
  if (!flags.dryRun) await writeEnv(deps.fs, flags.cwd, { ...existing, ...envFromAnswers(answers) });
  await applyPackages(
    flags,
    deps,
    ecosystem,
    entries,
    selected.filter((name) => !installed.includes(name)),
    installed.filter((name) => !selected.includes(name)),
    selected,
  );
  await applyMigrateInit(flags, deps, ecosystem, selected);
  const dialect: Dialect = migrateLanguage(ecosystem);
  await wireRegister(flags.cwd, "generated", dialect, flags.dryRun, deps);
  if (!answers.scaffold) return;
  if (flags.dryRun) {
    deps.log(typesPath);
    deps.log(datasourcePath);
    return;
  }
  await deps.fs.writeFile(typesPath, starterTypesYaml);
  if ((await optional(deps.fs, datasourcePath)) === undefined)
    await deps.fs.writeFile(datasourcePath, defaultDatasourceYaml);
};

const applyAddRemove = async (
  flags: Flags,
  deps: Deps,
  add: Provider[],
  remove: Provider[],
): Promise<void> => {
  const entries = await deps.fs.readdir(flags.cwd);
  const found = detect(entries);
  const ecosystem = resolveEcosystem(found, flags.ecosystem);
  const installed = await readInstalled(deps.fs, flags.cwd, ecosystem, entries);
  const remaining = [
    ...installed.filter((name) => !remove.includes(name)),
    ...add.filter((name) => !installed.includes(name)),
  ];
  const env = await readEnvFile(deps.fs, join(flags.cwd, ".env"));
  if (env && !flags.dryRun) await writeEnv(deps.fs, flags.cwd, updateEnvProviders(env, add, remove));
  await applyPackages(flags, deps, ecosystem, entries, add, remove, remaining);
  await applyMigrateInit(flags, deps, ecosystem, remaining);
};

const applyPackages = async (
  flags: Flags,
  deps: Deps,
  ecosystem: Ecosystem,
  entries: string[],
  add: Provider[],
  remove: Provider[],
  remaining: Provider[],
): Promise<void> => {
  const procs: Proc[] = [
    ...(add.length ? addCommands(ecosystem, add, entries) : []),
    ...(remove.length ? removeCommands(ecosystem, remove, remaining, entries) : []),
  ];
  for (const proc of procs) {
    deps.log(`$ ${proc.command} ${proc.args.join(" ")}`);
    if (!flags.dryRun) await deps.run(proc.command, proc.args, flags.cwd);
  }
  if (ecosystem === "rust") {
    for (const name of add) {
      const proc = { command: "cargo", args: ["build", "-p", catalog[name].cargo] };
      deps.log(`$ ${proc.command} ${proc.args.join(" ")}`);
      if (!flags.dryRun) {
        await deps.run(proc.command, proc.args, flags.cwd);
        const copied = await copyPlugin(flags.cwd, catalog[name].cargo);
        if (copied) deps.log(`copy ${copied}`);
      }
    }
  }
};

const applyMigrateInit = async (
  flags: Flags,
  deps: Deps,
  ecosystem: Ecosystem,
  remaining: Provider[],
): Promise<void> => {
  for (const proc of migrateInitCommands(sqlMigrateProviders(remaining), migrateLanguage(ecosystem))) {
    deps.log(`$ ${proc.command} ${proc.args.join(" ")}`);
    if (!flags.dryRun) await deps.run(proc.command, proc.args, flags.cwd);
  }
};

const readInstalled = async (
  fs: Fs,
  cwd: string,
  ecosystem: Ecosystem,
  entries: string[],
): Promise<Provider[]> =>
  ({
    ts: async () => {
      const text = await optional(fs, join(cwd, "package.json"));
      return text ? npmProviders(JSON.parse(text) as NpmManifest) : [];
    },
    rust: async () => {
      const text = await optional(fs, join(cwd, "Cargo.toml"));
      return text ? cargoProviders(text) : [];
    },
    csharp: async () => {
      const target = csharpTarget(entries);
      const text = target.endsWith(".csproj") ? await optional(fs, join(cwd, target)) : undefined;
      return text ? csharpProviders(text) : [];
    },
  })[ecosystem]();

const readEnv = async (fs: Fs, cwd: string): Promise<EnvMap> =>
  parseEnv(
    (await optional(fs, join(cwd, ".env"))) ??
      (await optional(fs, join(cwd, ".env.example"))) ??
      "",
  );

const readEnvFile = async (fs: Fs, path: string): Promise<EnvMap | undefined> => {
  const text = await optional(fs, path);
  return text === undefined ? undefined : parseEnv(text);
};

const writeEnv = (fs: Fs, cwd: string, env: EnvMap): Promise<void> =>
  fs.writeFile(join(cwd, ".env"), serializeEnv(env));

const optional = async (fs: Fs, path: string): Promise<string | undefined> => {
  try {
    return await fs.readFile(path);
  } catch {
    return undefined;
  }
};

export const pluginFileName = (cargoName: string, platform = process.platform): string => {
  const stem = cargoName.replaceAll("-", "_");
  return platform === "win32"
    ? `${stem}.dll`
    : platform === "darwin"
      ? `lib${stem}.dylib`
      : `lib${stem}.so`;
};

const copyPlugin = async (cwd: string, cargoName: string): Promise<string | undefined> => {
  const file = pluginFileName(cargoName);
  const source = await firstFile(
    [0, 1, 2].map((up) => join(cwd, ...Array.from({ length: up }, () => ".."), "target", "debug", file)),
  );
  if (!source) return undefined;
  const dest = join(cwd, "plugins", file);
  await mkdir(join(cwd, "plugins"), { recursive: true });
  await copyFile(source, dest);
  return dest;
};

const firstFile = async (paths: string[]): Promise<string | undefined> => {
  for (const path of paths) {
    try {
      await access(path);
      return path;
    } catch {
      continue;
    }
  }
  return undefined;
};
