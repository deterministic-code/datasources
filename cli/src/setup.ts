import { join } from "node:path";
import { isProvider, providers, requireProviders, type Ecosystem, type Provider } from "./catalog.ts";
import { addCommands, removeCommands, type Proc } from "./commands.ts";
import { requireDatasourcesRepo } from "./localRepo.ts";
import { csharpTarget, detect, isEcosystem, requireProjects, setupLanguages } from "./detect.ts";
import {
  answersFromEnv,
  ensureProviders,
  parseEnv,
  scheme,
  serializeEnv,
  splitProviders,
  updateEnvProviders,
  withSchema,
  type EnvMap,
} from "./env.ts";
import { flattenSettings } from "./generate/settingsFlat.ts";
import { defaultDatasourceYaml } from "./generate/loadSpec.ts";
import type { Deps, Fs, Prompter } from "./io.ts";
import { cargoProviders, csharpProviders, hasDatasourcesNpm, npmProviders, type NpmManifest } from "./installed.ts";
import { migrateInitCommands, migrateLanguage, sqlMigrateProviders } from "./migrateInit.ts";
import type { Flags } from "./parse.ts";
import { wireCsharpBuildProps } from "./wireCsharpBuildProps.ts";
import { wireRustProviders } from "./wireRustProviders.ts";

export type { Prompter } from "./io.ts";

export type SetupAnswers = {
  languages?: Ecosystem[];
  providers: string[];
  url: string;
  schema: string;
  scaffold: boolean;
};

export const starterTypesYaml = `version: 1.0.0
types:
  - item:
      tags:
        - datasource_type
      fields:
        - id:
            type: integer
            is_id: true
        - name:
            type: string
            size: 128
`;

export const answersForSetup = (env: EnvMap, requested: string[] = []) =>
  requested.length ? answersFromEnv({ ...env, INSTALLED_PROVIDERS: requested.join(",") }) : answersFromEnv(env);

export const promptSetup = async (
  prompt: Prompter,
  env: EnvMap,
  found: Ecosystem[],
  explicit: Ecosystem[] = [],
  askScaffold = false,
  requested: string[] = [],
): Promise<SetupAnswers> => {
  const defaults = answersForSetup(env, requested);
  const picked =
    explicit.length
      ? explicit
      : found.length === 1
        ? found
        : found.length === 0
          ? []
          : (await prompt.checkbox({
              message: "Languages",
              choices: found.map((value) => ({ name: value, value, checked: true })),
            })).filter(isEcosystem);
  const fromUrl = scheme(defaults.url);
  const selected = await prompt.checkbox({
    message: "Datasources",
    choices: providers.map((value) => ({
      name: value,
      value,
      checked: defaults.providers.includes(value) || value === fromUrl,
    })),
  });
  const url = await prompt.input({
    message: "DATASOURCE_URL",
    default:
      defaults.url || (selected.includes("sqlite") ? "sqlite:///:memory:" : ""),
  });
  const schema = await prompt.input({ message: "schema", default: defaults.schema });
  const scaffold = askScaffold
    ? await prompt.confirm({ message: "Create deterministic directory", default: true })
    : false;
  return { languages: picked.length ? picked : undefined, providers: selected, url, schema, scaffold };
};

export const envFromAnswers = (answers: SetupAnswers): EnvMap => {
  const provider = scheme(answers.url) ?? answers.providers[0] ?? "sqlite";
  const names = answers.providers.length
    ? isProvider(provider)
      ? ensureProviders(answers.providers, provider)
      : answers.providers
    : isProvider(provider)
      ? [provider]
      : ["sqlite"];
  return {
    INSTALLED_PROVIDERS: names.join(","),
    PROVIDER: provider,
    DATASOURCE_URL: withSchema(answers.url, answers.schema),
  };
};

export const applySetup = async (flags: Flags, deps: Deps, requested: string[]): Promise<void> => {
  const entries = await deps.fs.readdir(flags.cwd);
  const found = detect(entries);
  const existing = {
    ...(await readEnv(deps.fs, flags.cwd)),
    ...(process.env.INSTALLED_PROVIDERS ? { INSTALLED_PROVIDERS: process.env.INSTALLED_PROVIDERS } : {}),
    ...(process.env.PROVIDER ? { PROVIDER: process.env.PROVIDER } : {}),
    ...(process.env.DATASOURCE_URL ? { DATASOURCE_URL: process.env.DATASOURCE_URL } : {}),
  };
  const specDir = join(flags.cwd, "deterministic");
  const typesPath = join(specDir, "types.yaml");
  const datasourcePath = join(specDir, "datasource.yaml");
  const missingSpec = (await optional(deps.fs, typesPath)) === undefined;
  const answers: SetupAnswers = flags.yes
    ? await yesAnswers(existing, requested, flags.languages, deps.fs, specDir)
    : await promptSetup(deps.prompt, existing, found, flags.languages, missingSpec, requested);
  const explicit = answers.languages ?? flags.languages;
  const languages = setupLanguages(found, explicit, await npmHasDatasources(deps.fs, flags.cwd));
  requireProjects(languages, entries);
  const env = envFromAnswers(answers);
  const selected = requireProviders(splitProviders(env.INSTALLED_PROVIDERS));
  if (!flags.dryRun) await writeEnv(deps.fs, flags.cwd, { ...existing, ...env });
  for (const language of languages) {
    const installed = await readInstalled(deps.fs, flags.cwd, language, entries);
    await applyPackages(
      flags,
      deps,
      language,
      entries,
      selected,
      installed.filter((name) => !selected.includes(name)),
      selected,
    );
    await applyMigrateInit(flags, deps, language, selected);
  }
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

export const applyAddRemove = async (
  flags: Flags,
  deps: Deps,
  add: Provider[],
  remove: Provider[],
): Promise<void> => {
  const entries = await deps.fs.readdir(flags.cwd);
  const found = detect(entries);
  const languages = setupLanguages(found, flags.languages, await npmHasDatasources(deps.fs, flags.cwd));
  requireProjects(languages, entries);
  const env = await readEnvFile(deps.fs, join(flags.cwd, ".env"));
  if (env && !flags.dryRun) await writeEnv(deps.fs, flags.cwd, updateEnvProviders(env, add, remove));
  for (const language of languages) {
    const installed = await readInstalled(deps.fs, flags.cwd, language, entries);
    const remaining = [
      ...installed.filter((name) => !remove.includes(name)),
      ...add.filter((name) => !installed.includes(name)),
    ];
    await applyPackages(flags, deps, language, entries, add, remove, remaining);
    await applyMigrateInit(flags, deps, language, add);
  }
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
  const local = flags.local ? { repo: requireDatasourcesRepo(flags.cwd), cwd: flags.cwd } : undefined;
  const csproj = await csharpProject(flags.cwd, ecosystem, entries, deps.fs);
  const procs: Proc[] = [
    ...(add.length ? addCommands(ecosystem, add, entries, csproj, local) : []),
    ...(remove.length ? removeCommands(ecosystem, remove, remaining, entries, csproj, local) : []),
    ...(add.length ? csharpRestore(ecosystem, entries) : []),
  ];
  await runProcs(flags, deps, procs);
  await {
    rust: () => wireRustProviders(flags.cwd, remaining, flags.dryRun, deps),
    csharp: () => wireCsharpBuildProps(flags.cwd, flags.dryRun, deps),
    ts: () => Promise.resolve(),
  }[ecosystem]();
};

const yesAnswers = async (
  existing: EnvMap,
  requested: string[],
  languages: Ecosystem[],
  fs: Fs,
  specDir: string,
): Promise<SetupAnswers> => {
  const base = answersForSetup(existing, requested);
  return {
    ...base,
    providers: [...new Set([...base.providers, ...(await settingsDatasources(fs, specDir))])],
    languages,
    scaffold: false,
  };
};

const settingsDatasources = async (fs: Fs, specDir: string): Promise<string[]> => {
  const text = await optional(fs, join(specDir, "settings.yaml"));
  return text === undefined ? [] : splitProviders(flattenSettings(text)["backend.datasources"] ?? "");
};

const csharpRestore = (ecosystem: Ecosystem, entries: string[]): Proc[] => {
  if (ecosystem !== "csharp") return [];
  const project = csharpTarget(entries);
  return [{ command: "dotnet", args: ["restore", project] }];
};

const applyMigrateInit = async (
  flags: Flags,
  deps: Deps,
  ecosystem: Ecosystem,
  remaining: Provider[],
): Promise<void> => {
  await runProcs(flags, deps, migrateInitCommands(sqlMigrateProviders(remaining), migrateLanguage(ecosystem)));
};

const npmHasDatasources = async (fs: Fs, cwd: string): Promise<boolean> => {
  const text = await optional(fs, join(cwd, "package.json"));
  return text !== undefined && hasDatasourcesNpm(JSON.parse(text) as NpmManifest);
};

const npmProject = `${JSON.stringify({ private: true }, null, 2)}\n`;

const ensureNpmProject = async (flags: Flags, deps: Deps): Promise<void> => {
  const path = join(flags.cwd, "package.json");
  if ((await optional(deps.fs, path)) !== undefined) return;
  deps.log(path);
  if (!flags.dryRun) await deps.fs.writeFile(path, npmProject);
};

const runProcs = async (flags: Flags, deps: Deps, procs: Proc[]): Promise<void> => {
  if (procs.some((proc) => proc.command === "npm")) await ensureNpmProject(flags, deps);
  for (const proc of procs) {
    deps.log(`$ ${proc.command} ${proc.args.join(" ")}`);
    if (!flags.dryRun) await deps.run(proc.command, proc.args, flags.cwd);
  }
};

const csharpProject = async (
  cwd: string,
  ecosystem: Ecosystem,
  entries: string[],
  fs: Fs,
): Promise<string | undefined> => {
  if (ecosystem !== "csharp") return undefined;
  const target = csharpTarget(entries);
  return target.endsWith(".csproj") ? optional(fs, join(cwd, target)) : undefined;
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
