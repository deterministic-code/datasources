import { providers, type Ecosystem } from "./catalog.ts";
import { isEcosystem } from "./detect.ts";
import { answersFromEnv, ensureProviders, type EnvMap } from "./env.ts";

export type SetupAnswers = {
  ecosystem?: Ecosystem;
  provider: string;
  providers: string[];
  filename: string;
  url: string;
  schema: string;
  engine: boolean;
};

export type Prompter = {
  select: (opts: {
    message: string;
    choices: { name: string; value: string }[];
    default: string;
  }) => Promise<string>;
  checkbox: (opts: {
    message: string;
    choices: { name: string; value: string; checked: boolean }[];
  }) => Promise<string[]>;
  input: (opts: { message: string; default: string }) => Promise<string>;
  confirm: (opts: { message: string; default: boolean }) => Promise<boolean>;
};

export const promptSetup = async (
  prompt: Prompter,
  env: EnvMap,
  found: Ecosystem[],
  explicit?: Ecosystem,
): Promise<SetupAnswers> => {
  const defaults = answersFromEnv(env);
  const ecosystem =
    explicit ??
    (found.length === 1
      ? found[0]
      : found.length === 0
        ? undefined
        : ecosystemOr(await prompt.select({
            message: "Project type",
            choices: found.map((value) => ({ name: value, value })),
            default: found[0]!,
          })));
  const provider = await prompt.select({
    message: "Default provider",
    choices: providers.map((value) => ({ name: value, value })),
    default: defaults.provider,
  });
  const selected = ensureProviders(
    await prompt.checkbox({
      message: "Providers",
      choices: providers.map((value) => ({
        name: value,
        value,
        checked: defaults.providers.includes(value) || value === provider,
      })),
    }),
    provider,
  );
  const filename = selected.includes("sqlite")
    ? await prompt.input({ message: "SQLITE_FILENAME", default: defaults.filename })
    : defaults.filename;
  const url = await prompt.input({ message: "DATABASE_URL", default: defaults.url });
  const schema = await prompt.input({ message: "DATABASE_SCHEMA", default: defaults.schema });
  const engine = selected.some((name) => name === "sqlite" || name === "mysql" || name === "postgres")
    ? await prompt.confirm({ message: "Install native engines", default: true })
    : false;
  return { ecosystem, provider, providers: selected, filename, url, schema, engine };
};

const ecosystemOr = (value: string): Ecosystem | undefined =>
  isEcosystem(value) ? value : undefined;

export const envFromAnswers = (answers: SetupAnswers): EnvMap => ({
  DATASOURCE_PROVIDER: answers.provider,
  DATASOURCE_PROVIDERS: ensureProviders(answers.providers, answers.provider).join(","),
  SQLITE_FILENAME: answers.filename,
  DATABASE_URL: answers.url,
  DATABASE_SCHEMA: answers.schema,
});
