import { providers, type Ecosystem } from "./catalog.ts";
import { isEcosystem } from "./detect.ts";
import { answersFromEnv, ensureProviders, type EnvMap } from "./env.ts";
import { providerFromUrl } from "./providerFromUrl.ts";

export type SetupAnswers = {
  ecosystem?: Ecosystem;
  providers: string[];
  url: string;
  schema: string;
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
  const fromUrl = providerFromUrl(defaults.url);
  const selected = await prompt.checkbox({
    message: "Providers",
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
  const schema = await prompt.input({ message: "DATABASE_SCHEMA", default: defaults.schema });
  return { ecosystem, providers: selected, url, schema };
};

const ecosystemOr = (value: string): Ecosystem | undefined =>
  isEcosystem(value) ? value : undefined;

export const envFromAnswers = (answers: SetupAnswers): EnvMap => ({
  INSTALLED_PROVIDERS: ensureProviders(
    answers.providers,
    providerFromUrl(answers.url) ?? answers.providers[0] ?? "sqlite",
  ).join(","),
  DATASOURCE_URL: answers.url,
  DATABASE_SCHEMA: answers.schema,
});
