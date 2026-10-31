import { StatementNaming, type Casing } from "@deterministic-code/datasources";
import type { ICasingStrategy } from "@deterministic-code/generators-common/casing-strategy";
import type { IDeterministic } from "@deterministic-code/deterministic-generators-common/parser";
import type { GenerateEntry } from "@deterministic-code/generators-common/generate-entry";
import type { ILanguageRepositories } from "./Language.ts";
import {
  childClass,
  childField,
  children as childModels,
  dialectsOf,
  isLinked,
  type OptimisticConcurrency,
  type PlannedEnrichment,
  type RepositoryModel,
  type RepositoryOptionsModel,
} from "./RepositoryModel.ts";

export abstract class Repositories implements ILanguageRepositories {
  abstract generate(models: RepositoryModel[], settings: Record<string, string>, spec: IDeterministic): GenerateEntry[];

  protected quote(value: string): string {
    return value.replaceAll("\\", "\\\\").replaceAll("\"", "\\\"");
  }

  protected plural(name: string, casing: Casing): string {
    const naming = StatementNaming.sql;
    return naming.convert(naming.physicalStem(naming.convert(name, "snakeCase")), casing);
  }

  protected listed(names: string[]): Array<{ name: string; comma?: Record<string, never> }> {
    return names.map((name, index) => ({ name: this.quote(name), comma: index > 0 ? {} : undefined }));
  }

  protected optionsOf(model: RepositoryOptionsModel & { entity: string }): Record<string, unknown> {
    return {
      entity: this.quote(model.entity),
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
    };
  }

  protected dialectsBag(model: RepositoryModel): Record<string, unknown>[] {
    return dialectsOf(model).map(([engine, planned]) => ({
      engine: this.quote(engine),
      ...this.optionsOf({ entity: model.entity, ...planned }),
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
      className: childClass(child.typeName),
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
