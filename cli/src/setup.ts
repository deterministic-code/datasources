import { providers, type Ecosystem } from "./catalog.ts";
import { isEcosystem } from "./detect.ts";
import { answersFromEnv, ensureProviders, scheme, withSchema, type EnvMap } from "./env.ts";

export type SetupAnswers = {
  languages?: Ecosystem[];
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
