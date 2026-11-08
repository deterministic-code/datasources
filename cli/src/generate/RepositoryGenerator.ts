import type { ICasingStrategy } from "@deterministic-code/generators-common/casing-strategy";
import type { IDeterministic, TypeField } from "@deterministic-code/deterministic-generators-common/parser";
import type { GenerateEntry } from "@deterministic-code/generators-common/generate-entry";
import { Paths, type TypeKind } from "@deterministic-code/generators-common/paths";
import { caseDirectories } from "./casing.ts";
import type { IRepositoryGenerator } from "./IRepositoryGenerator.ts";
import {
  childField,
  children as childModels,
  dialectsOf,
  isLinked,
  type OptimisticConcurrency,
  type PlannedEnrichment,
  type RepositoryModel,
  type RepositoryOptionsModel,
} from "./RepositoryModel.ts";

const languages: Record<string, () => Promise<IRepositoryGenerator>> = {
  csharp: () => import("./CSharpRepositories.ts").then((mod) => mod.csharpRepositories),
  rust: () => import("./RustRepositories.ts").then((mod) => mod.rustRepositories),
  typescript: () => import("./TypeScriptRepositories.ts").then((mod) => mod.typescriptRepositories),
};

export const languageOf = async (dialect: string): Promise<IRepositoryGenerator> => {
  const load = languages[dialect];
  if (load === undefined) throw new Error(`Unknown dialect: ${dialect}`);
  return load();
};

export abstract class RepositoryGenerator implements IRepositoryGenerator {
  abstract generate(models: RepositoryModel[], settings: Record<string, string>, spec: IDeterministic): GenerateEntry[];
  abstract readonly types: IRepositoryGenerator["types"];
  protected readonly paths = new Paths();

  typesPrefix = (casing: ICasingStrategy, kind: TypeKind): string =>
    caseDirectories(this.paths.typesDir(kind), casing);

  protected quote(value: string): string {
    return value.replaceAll("\\", "\\\\").replaceAll("\"", "\\\"");
  }

  protected repositoryClass(specName: string, casing: ICasingStrategy): string {
    return casing.convertTypes(this.paths.repositoryStem(specName));
  }

  protected repositoryFile(specName: string, casing: ICasingStrategy): string {
    return casing.convertFileName(this.paths.repositoryStem(specName));
  }

  protected bindStem(specName: string): string {
    return this.paths.repositoryStem(specName).slice(0, -"_repository".length);
  }

  protected bindName(specName: string, casing: ICasingStrategy, as: "fields" | "types"): string {
    const stem = this.bindStem(specName);
    return as === "fields" ? casing.convertFields(stem) : casing.convertTypes(stem);
  }

  protected rowRepositoryStem(specName: string): string {
    return `${this.bindStem(specName)}_row_repository`;
  }

  protected readonlyRepositoryStem(specName: string): string {
    return `${this.bindStem(specName)}_readonly_repository`;
  }

  protected listed(names: string[]): Array<{ name: string; comma?: Record<string, never> }> {
    return names.map((name, index) => ({ name: this.quote(name), comma: index > 0 ? {} : undefined }));
  }

  protected pairList(pairs: Array<{ from: string; to: string }>): string {
    return `[${pairs.map((pair) => `["${this.quote(pair.from)}", "${this.quote(pair.to)}"]`).join(", ")}]`;
  }

  protected schemaBag(fields: TypeField[], casing: ICasingStrategy): Record<string, unknown>[] {
    return fields.map((field, index) => {
      const name = this.quote(casing.convertFields(field.name));
      return {
        from: name,
        name,
        type: this.quote(field.type),
        to: this.quote(field.type),
        nullable: field.isNullable ? {} : undefined,
        hasDefault: field.defaultValue !== undefined && field.defaultValue !== null ? {} : undefined,
        isId: field.isId === true ? {} : undefined,
        first: index === 0 ? {} : undefined,
        last: index === fields.length - 1 ? {} : undefined,
        comma: index > 0 ? {} : undefined,
      };
    });
  }

  protected optionsOf(model: RepositoryOptionsModel & { typeName: string }): Record<string, unknown> {
    return {
      table: this.quote(model.table),
      ids: this.listed(model.ids),
      readProperties: this.listed(model.readProperties),
      createProperties: this.listed(model.createProperties),
      updateProperties: this.listed(model.updateProperties),
      propertyMappings: model.propertyMappings.map((pair, index) => ({
        from: this.quote(pair.from),
        to: this.quote(pair.to),
        first: index === 0 ? {} : undefined,
        last: index === model.propertyMappings.length - 1 ? {} : undefined,
        comma: index > 0 ? {} : undefined,
      })),
      entityPairs: this.pairList([{ from: model.typeName, to: model.table }]),
      mappingPairs: this.pairList(model.propertyMappings),
    };
  }

  protected dialectsBag(model: RepositoryModel, casing: ICasingStrategy): Record<string, unknown>[] {
    return dialectsOf(model).map(([engine, planned]) => ({
      engine: this.quote(engine),
      ...this.optionsOf({ typeName: model.typeName, ...planned }),
      schema: this.schemaBag(model.schema, casing),
    }));
  }

  protected enrichmentOf(item: PlannedEnrichment, casing: ICasingStrategy): Record<string, unknown> {
    return {
      destination: this.quote(item.destination),
      source: this.quote(item.source),
      relatedKey: this.quote(item.relatedKey),
      relatedProperty: item.relatedProperty === undefined ? undefined : this.quote(item.relatedProperty),
      via: childField(casing, item.via),
      viaType: item.viaType,
      object: item.kind === "object" ? {} : undefined,
      dictionary: item.kind === "dictionary" ? {} : undefined,
      list: item.kind === "list" || item.kind === "linked" ? {} : undefined,
      linked: isLinked(item)
        ? {
            link: childField(casing, item.link),
            parentFk: this.quote(item.parentFk),
            relatedFk: this.quote(item.relatedFk),
          }
        : undefined,
    };
  }

  protected childrenOf(model: RepositoryModel, casing: ICasingStrategy): Array<{ specName: string; typeName: string; className: string; field: string }> {
    return childModels(model).map((child) => ({
      specName: child.specName,
      typeName: child.typeName,
      className: this.repositoryClass(child.specName, casing),
      field: childField(casing, child.specName),
    }));
  }

  protected whereKeys(model: RepositoryModel, token: OptimisticConcurrency): Record<string, unknown>[] {
    return [...model.ids, token.property].map((name, index) => ({
      name: this.quote(name),
      token: name === token.property ? {} : undefined,
      first: index === 0 ? {} : undefined,
      rest: index > 0 ? {} : undefined,
    }));
  }
}
