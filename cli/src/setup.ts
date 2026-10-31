import { join } from "node:path";
import { providers, requireProviders, type Ecosystem, type Provider } from "./catalog.ts";
import { addCommands, removeCommands, type Proc } from "./commands.ts";
import { csharpTarget, detect, isEcosystem, requireProjects, resolveLanguages } from "./detect.ts";
import { answersFromEnv, ensureProviders, parseEnv, scheme, serializeEnv, updateEnvProviders, withSchema, type EnvMap } from "./env.ts";
import { defaultDatasourceYaml } from "./generate/loadSpec.ts";
import type { Deps, Fs, Prompter } from "./io.ts";
import { cargoProviders, csharpProviders, npmProviders, type NpmManifest } from "./installed.ts";
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
  return {
    INSTALLED_PROVIDERS: ensureProviders(answers.providers, provider).join(","),
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
    ? { ...answersForSetup(existing, requested), languages: flags.languages, scaffold: false }
    : await promptSetup(deps.prompt, existing, found, flags.languages, missingSpec, requested);
  const languages = resolveLanguages(found, answers.languages ?? flags.languages);
  requireProjects(languages, entries);
  const selected = requireProviders(answers.providers);
  if (!flags.dryRun) await writeEnv(deps.fs, flags.cwd, { ...existing, ...envFromAnswers(answers) });
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
  const languages = resolveLanguages(found, flags.languages);
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
    await applyMigrateInit(flags, deps, language, remaining);
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
  const procs: Proc[] = [
    ...(add.length ? addCommands(ecosystem, add, entries, await csharpProject(flags.cwd, ecosystem, entries, deps.fs)) : []),
    ...(remove.length ? removeCommands(ecosystem, remove, remaining, entries) : []),
  ];
  for (const proc of procs) {
    deps.log(`$ ${proc.command} ${proc.args.join(" ")}`);
    if (!flags.dryRun) await deps.run(proc.command, proc.args, flags.cwd);
  }
  await {
    rust: () => wireRustProviders(flags.cwd, remaining, flags.dryRun, deps),
    csharp: () => wireCsharpBuildProps(flags.cwd, flags.dryRun, deps),
    ts: () => Promise.resolve(),
  }[ecosystem]();
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
