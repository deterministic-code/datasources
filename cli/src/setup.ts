import { providers, type Ecosystem } from "./catalog.ts";
import { isEcosystem } from "./detect.ts";
import { answersFromEnv, ensureProviders, scheme, withSchema, type EnvMap } from "./env.ts";

export type SetupAnswers = {
  ecosystem?: Ecosystem;
  providers: string[];
  url: string;
  schema: string;
  scaffold: boolean;
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

export const promptSetup = async (
  prompt: Prompter,
  env: EnvMap,
  found: Ecosystem[],
  explicit?: Ecosystem,
  askScaffold = false,
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
  const fromUrl = scheme(defaults.url);
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
  const schema = await prompt.input({ message: "schema", default: defaults.schema });
  const scaffold = askScaffold
    ? await prompt.confirm({ message: "Create deterministic directory", default: true })
    : false;
  return { ecosystem, providers: selected, url, schema, scaffold };
};

const ecosystemOr = (value: string): Ecosystem | undefined =>
  isEcosystem(value) ? value : undefined;

export const envFromAnswers = (answers: SetupAnswers): EnvMap => {
  const provider = scheme(answers.url) ?? answers.providers[0] ?? "sqlite";
  return {
    INSTALLED_PROVIDERS: ensureProviders(answers.providers, provider).join(","),
    PROVIDER: provider,
    DATASOURCE_URL: withSchema(answers.url, answers.schema),
  };
};
