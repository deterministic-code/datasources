import { generate as csharpDatasourceTypes } from "@deterministic-code/csharp-generators/generate-datasource-types";
import { generate as csharpViewTypes } from "@deterministic-code/csharp-generators/generate-view-types";
import type { GenerateContext } from "@deterministic-code/generators-common/generate-context";
import type { GenerateEntry } from "@deterministic-code/generators-common/generate-entry";
import { generateSql } from "@deterministic-code/datasources-code-generators";
import { generate as rustDatasourceTypes } from "@deterministic-code/rust-generators/generate-datasource-types";
import { generate as rustViewTypes } from "@deterministic-code/rust-generators/generate-view-types";
import { generate as typescriptDatasourceTypes } from "@deterministic-code/typescript-generators/generate-datasource-types";
import { generate as typescriptViewTypes } from "@deterministic-code/typescript-generators/generate-view-types";
import { DeterministicParser } from "@deterministic-code/deterministic-generators-common/parser";
import type { CodeGenerationProvider } from "./CodeGenerationProvider.ts";
import type { Dialect, GenerateRequest } from "./GenerateRequest.ts";
import type { RepositoryModel } from "./RepositoryModel.ts";
import { csharpRepositories } from "./CSharpRepositories.ts";
import { rustRepositories } from "./RustRepositories.ts";
import { typescriptRepositories } from "./TypeScriptRepositories.ts";
import { planRepositories } from "./RepositoryPlan.ts";

const place = (dir: string, entries: GenerateEntry[]): GenerateEntry[] =>
  entries.map((entry) => ({ ...entry, filename: `${dir}/${entry.filename}` }));

const typePacks: Record<Dialect, { datasource: (ctx: GenerateContext) => Promise<GenerateEntry[]>; views: (ctx: GenerateContext) => Promise<GenerateEntry[]> }> = {
  csharp: { datasource: csharpDatasourceTypes, views: csharpViewTypes },
  typescript: { datasource: typescriptDatasourceTypes, views: typescriptViewTypes },
  rust: { datasource: rustDatasourceTypes, views: rustViewTypes },
};

const repositoryPacks: Record<Dialect, (models: RepositoryModel[], settings: Record<string, string>) => GenerateEntry[]> = {
  csharp: csharpRepositories,
  typescript: typescriptRepositories,
  rust: rustRepositories,
};

const context = (request: GenerateRequest): GenerateContext => ({ reader: request.reader, settings: request.settings });

const dialectsOf = (settings: Record<string, string>): string[] => {
  const out: string[] = [];
  for (const name of (settings["backend.datasources"] ?? "").split(",")) {
    const key = name.trim();
    if (key !== "" && !out.includes(key)) out.push(key);
  }
  return out;
};

export const deterministic: CodeGenerationProvider = {
  name: "deterministic",
  generateTypes: async (request) => {
    const pack = typePacks[request.dialect];
    const ctx = context(request);
    const datasource = await pack.datasource(ctx);
    const views = await pack.views(ctx);
    return request.dialect === "typescript"
      ? [
          ...place("typescript/types/generated/datasource", datasource),
          ...place("typescript/types/generated/views", views),
        ]
      : place(request.dialect, [...datasource, ...views]);
  },
  generateRepositories: async (request) => {
    const spec = await DeterministicParser(request.reader).parse(request.settings);
    return place(`${request.dialect}/repositories`, repositoryPacks[request.dialect](planRepositories(spec, request.dialect, request.settings), request.settings));
  },
  generateDatasource: async (request) =>
    place("sql", await generateSql(context(request), dialectsOf(request.settings))),
};
