import { generate as csharpDatasourceTypes } from "@deterministic-code/csharp-generators/generate-datasource-types";
import { generate as csharpViewTypes } from "@deterministic-code/csharp-generators/generate-view-types";
import type { GenerateContext } from "@deterministic-code/generators-common/generate-context";
import type { GenerateEntry } from "@deterministic-code/generators-common/generate-entry";
import { generateSql } from "@deterministic-code/datasources-code-generators";
import { generate as rustDatasourceTypes } from "@deterministic-code/rust-generators/generate-datasource-types";
import { generate as rustViewTypes } from "@deterministic-code/rust-generators/generate-view-types";
import { generate as typescriptDatasourceTypes } from "@deterministic-code/typescript-generators/generate-datasource-types";
import { generate as typescriptViewTypes } from "@deterministic-code/typescript-generators/generate-view-types";
import { createCasingStrategy } from "@deterministic-code/generators-common/casing-strategy";
import { DeterministicParser, type IDeterministic } from "@deterministic-code/deterministic-generators-common/parser";
import { caseDirectories, repositoriesPrefix, typesPrefix } from "./casing.ts";
import type { CodeGenerationProvider } from "./CodeGenerationProvider.ts";
import type { Dialect, GenerateRequest } from "./GenerateRequest.ts";
import type { RepositoryModel, RepositoryOptionsModel } from "./RepositoryModel.ts";
import { csharpRepositories } from "./CSharpRepositories.ts";
import { rustRepositories } from "./RustRepositories.ts";
import { typescriptRepositories } from "./TypeScriptRepositories.ts";
import { planRepositories } from "./RepositoryPlan.ts";
import { datasourceEnginesOf, engineDatasourceOf, overlayDatasource } from "./datasourceYaml.ts";
import { sqlDialectsFromEnv } from "./sqlDialects.ts";

const place = (dir: string, entries: GenerateEntry[]): GenerateEntry[] =>
  entries.map((entry) => ({ ...entry, filename: `${dir}/${entry.filename}` }));

const remap = (entries: GenerateEntry[], settings: Record<string, string>, dialect: Dialect): GenerateEntry[] => {
  const casing = createCasingStrategy(dialect, settings);
  return entries.map((entry) => ({ ...entry, filename: caseDirectories(entry.filename, casing) }));
};

const typePacks: Record<Dialect, { datasource: (ctx: GenerateContext) => Promise<GenerateEntry[]>; views: (ctx: GenerateContext) => Promise<GenerateEntry[]> }> = {
  csharp: { datasource: csharpDatasourceTypes, views: csharpViewTypes },
  typescript: { datasource: typescriptDatasourceTypes, views: typescriptViewTypes },
  rust: { datasource: rustDatasourceTypes, views: rustViewTypes },
};

const repositoryPacks: Record<
  Dialect,
  (models: RepositoryModel[], settings: Record<string, string>, spec: IDeterministic) => GenerateEntry[]
> = {
  csharp: csharpRepositories,
  typescript: typescriptRepositories,
  rust: rustRepositories,
};

const context = (request: GenerateRequest): GenerateContext => ({ reader: request.reader, settings: request.settings });

const optionsOf = (model: RepositoryModel): RepositoryOptionsModel => ({
  table: model.table,
  ids: model.ids,
  readProperties: model.readProperties,
  createProperties: model.createProperties,
  updateProperties: model.updateProperties,
  propertyMappings: model.propertyMappings,
});

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
            optionsOf(planned.find((item) => item.specName === model.specName) ?? model),
          ]),
        ),
      }));
};

export const deterministic: CodeGenerationProvider = {
  name: "deterministic",
  generateTypes: async (request) => {
    const pack = typePacks[request.dialect];
    const ctx = context(request);
    const casing = createCasingStrategy(request.dialect, request.settings);
    const datasource = await pack.datasource(ctx);
    const views = await pack.views(ctx);
    const types = [
      ...place(typesPrefix(request.dialect, casing, "datasource"), datasource),
      ...place(typesPrefix(request.dialect, casing, "views"), views),
    ];
    if (request.dialect === "rust") {
      for (const entry of types) {
        entry.contents = entry.contents!.replace(/crate::types::generated::(?:views|datasource)::/g, "");
        if (!entry.contents.includes("pub struct")) continue;
        const extra = entry.contents.includes("impl Default") ? "" : ", Default";
        entry.contents = entry.contents
          .replace(
            /#\[derive\(([^)]*)\)\]/g,
            (_, list: string) =>
              `#[derive(${list}${extra}, datasources::Facet)]\n#[facet(crate = ::datasources::facet)]`,
          )
          .replace(/(^|[^:\w])chrono::/g, "$1datasources::chrono::")
          .replace(/(^|[^:\w])uuid::/g, "$1datasources::uuid::")
          .replace(/(^|[^:\w])rust_decimal::/g, "$1datasources::rust_decimal::");
      }
    }
    if (request.dialect === "typescript") {
      for (const entry of types) {
        entry.contents = entry.contents!.replace(
          /export interface (\w+)/g,
          (_, name: string) =>
            `export const ${name[0]!.toLowerCase()}${name.slice(1)} = (init: Partial<${name}>): ${name} => ({ ...init } as ${name});\nexport interface ${name}`,
        );
      }
    }
    return remap(
      request.dialect === "rust"
        ? types.filter((entry) => !/pub type (\w+) = \1;/.test(entry.contents!))
        : types,
      request.settings,
      request.dialect,
    );
  },
  generateRepositories: async (request) => {
    const spec = await DeterministicParser(request.reader).parse(request.settings);
    const models = planRepositories(spec, request.dialect, request.settings);
    const casing = createCasingStrategy(request.dialect, request.settings);
    return remap(
      place(
        repositoriesPrefix(request.dialect, casing),
        repositoryPacks[request.dialect](await withProviderWrites(request, models), request.settings, spec),
      ),
      request.settings,
      request.dialect,
    );
  },
  generateDatasource: async (request) =>
    place("sql", await generateSql(context(request), sqlDialectsFromEnv(request.env ?? {}))),
};
