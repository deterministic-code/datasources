import type { GenerateContext } from "@deterministic-code/generators-common/generate-context";
import type { GenerateEntry } from "@deterministic-code/generators-common/generate-entry";
import { generateSql } from "@deterministic-code/datasources-code-generators";
import { createCasingStrategy } from "@deterministic-code/generators-common/casing-strategy";
import { DeterministicParser } from "@deterministic-code/deterministic-generators-common/parser";
import { caseDirectories } from "./casing.ts";
import type { CodeGenerationProvider } from "./CodeGenerationProvider.ts";
import type { Dialect, GenerateRequest } from "./GenerateRequest.ts";
import type { RepositoryModel } from "./RepositoryModel.ts";
import { languageOf } from "./Language.ts";
import { planRepositories } from "./RepositoryPlan.ts";
import { datasourceEnginesOf, engineDatasourceOf, overlayDatasource } from "./datasourceYaml.ts";
import { sqlDialectsFromEnv } from "./sqlDialects.ts";

const place = (dir: string, entries: GenerateEntry[]): GenerateEntry[] =>
  entries.map((entry) => ({ ...entry, filename: `${dir}/${entry.filename}` }));

const remap = (entries: GenerateEntry[], settings: Record<string, string>, dialect: Dialect): GenerateEntry[] => {
  const casing = createCasingStrategy(dialect, settings);
  return entries.map((entry) => ({ ...entry, filename: caseDirectories(entry.filename, casing) }));
};

const context = (request: GenerateRequest): GenerateContext => ({ reader: request.reader, settings: request.settings });

const withProviderWrites = async (request: GenerateRequest, models: RepositoryModel[]): Promise<RepositoryModel[]> => {
  const byProvider: Record<string, RepositoryModel[]> = {};
  for (const engine of await datasourceEnginesOf(request.reader, request.settings)) {
    const yaml = await engineDatasourceOf(request.reader, engine);
    if (yaml === undefined) continue;
    byProvider[engine] = planRepositories(
      await DeterministicParser(overlayDatasource(request.reader, yaml)).parse(request.settings),
      request.dialect,
      request.settings,
    );
  }
  return Object.keys(byProvider).length === 0
    ? models
    : models.map((model) => ({
        ...model,
        optionsByProvider: Object.fromEntries(
          Object.entries(byProvider).map(([engine, planned]) => [
            engine,
            planned.find((item) => item.specName === model.specName) ?? model,
          ]),
        ),
      }));
};

export const deterministic: CodeGenerationProvider = {
  name: "deterministic",
  generateTypes: async (request) => {
    const language = await languageOf(request.dialect);
    const ctx = context(request);
    const casing = createCasingStrategy(request.dialect, request.settings);
    return remap(
      [
        ...place(language.typesPrefix(casing, "datasource"), await language.types.datasource(ctx)),
        ...place(language.typesPrefix(casing, "views"), await language.types.views(ctx)),
      ],
      request.settings,
      request.dialect,
    );
  },
  generateRepositories: async (request) => {
    const language = await languageOf(request.dialect);
    const spec = await DeterministicParser(request.reader).parse(request.settings);
    const models = planRepositories(spec, request.dialect, request.settings);
    const casing = createCasingStrategy(request.dialect, request.settings);
    return remap(
      place(
        language.repositoriesPrefix(casing),
        language.repositories.generate(await withProviderWrites(request, models), request.settings, spec),
      ),
      request.settings,
      request.dialect,
    );
  },
  generateDatasource: async (request) =>
    place("sql", await generateSql(context(request), sqlDialectsFromEnv(request.env ?? {}))),
};
