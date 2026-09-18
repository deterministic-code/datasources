import { join } from "node:path";
import { requireProviders, type Ecosystem, type Provider } from "./catalog.ts";
import { addCommands, removeCommands, type Proc } from "./commands.ts";
import { csharpTarget, detect, resolveEcosystem } from "./detect.ts";
import {
  answersFromEnv,
  parseEnv,
  serializeEnv,
  updateEnvProviders,
  type EnvMap,
} from "./env.ts";
import { envFromAnswers } from "./setup.ts";
import type { Fs, Runner } from "./io.ts";
import {
  cargoProviders,
  csharpProviders,
  npmProviders,
  withRegisterFeatures,
  type NpmManifest,
} from "./installed.ts";
import { help, parseArgs, type Command, type Flags } from "./parse.ts";
import { promptSetup, type Prompter, type SetupAnswers } from "./setup.ts";

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

const dispatch = async (command: Command, flags: Flags, deps: Deps): Promise<void> =>
  command.kind === "help"
    ? deps.log(help.trimEnd())
    : command.kind === "setup"
      ? applySetup(flags, deps)
      : command.kind === "add"
        ? applyAddRemove(flags, deps, requireProviders(needNames(command.providers, "add")), [])
        : applyAddRemove(flags, deps, [], requireProviders(needNames(command.providers, "remove")));

const needNames = (names: string[], verb: string): string[] =>
  names.length
    ? names
    : (() => {
        throw new Error(`Usage: datasources ${verb} <provider...>`);
      })();

const applySetup = async (flags: Flags, deps: Deps): Promise<void> => {
  const entries = await deps.fs.readdir(flags.cwd);
  const found = detect(entries);
  const existing = await readEnv(deps.fs, flags.cwd);
  const answers: SetupAnswers = flags.yes
    ? { ...answersFromEnv(existing), ecosystem: flags.ecosystem }
    : await promptSetup(deps.prompt, existing, found, flags.ecosystem);
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
  if (ecosystem === "rust" && (add.length || remove.length)) {
    deps.log(`update datasources-register features: ${remaining.join(",") || "(none)"}`);
    if (!flags.dryRun) await patchRegisterFeatures(deps.fs, flags.cwd, remaining);
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

const patchRegisterFeatures = async (fs: Fs, cwd: string, features: Provider[]): Promise<void> => {
  const path = join(cwd, "Cargo.toml");
  const text = await optional(fs, path);
  if (text) await fs.writeFile(path, withRegisterFeatures(text, features));
};
