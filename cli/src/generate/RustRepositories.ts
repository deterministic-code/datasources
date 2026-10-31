import { createCasingStrategy, type ICasingStrategy } from "@deterministic-code/generators-common/casing-strategy";
import { authoredViewTypesOf, datasourceTypesOf } from "@deterministic-code/generators-common/spec-types";
import type { IDeterministic } from "@deterministic-code/deterministic-generators-common/parser";
import { fill } from "@deterministic-code/generators-common/fill";
import { content, type GenerateEntry } from "@deterministic-code/generators-common/generate-entry";
import { dialectsOf, type RepositoryModel } from "./RepositoryModel.ts";
import { Repositories } from "./Repositories.ts";
import { schemaPairs } from "./schemaPairs.ts";
import { crateModTmpl, modTmpl, registryTmpl, repositoryTmpl, schemaObjectsTmpl } from "./templates/rust.ts";

class RustRepositories extends Repositories {
  generate(models: RepositoryModel[], settings: Record<string, string>, spec: IDeterministic): GenerateEntry[] {
    const casing = createCasingStrategy("rust", settings);
    const typeStems = [...new Set([...datasourceTypesOf(spec), ...authoredViewTypesOf(spec)].map((type) => casing.convertFileName(type.name)))].map((stem) => this.mod(stem));
    return [
      ...models.map((model) => content(`${casing.convertFileName(`${model.specName}_repository`)}.rs`, fill(repositoryTmpl, this.repository(model, spec, casing)))),
      content(`${casing.convertFileName("repository_registry")}.rs`, fill(registryTmpl, this.registry(models, casing))),
      content("mod.rs", fill(modTmpl, { mods: [...models.map((model) => this.mod(casing.convertFileName(`${model.specName}_repository`))), this.mod(casing.convertFileName("repository_registry"))] })),
      content("../mod.rs", fill(crateModTmpl, { typeStems, registryIdent: this.ident(casing.convertFileName("repository_registry")) })),
      content("../schema_objects.rs", fill(schemaObjectsTmpl, this.schemaTypes(models))),
    ];
  }

  private repository(model: RepositoryModel, spec: IDeterministic, casing: ICasingStrategy): Record<string, unknown> {
    const child = this.childrenOf(model, casing).map((item, index) => ({
      ...item,
      module: this.ident(casing.convertFileName(`${item.specName}_repository`)),
      rowFn: casing.convertFields(`${item.specName}_row_repository`),
      comma: index > 0 ? {} : undefined,
    }));
    const enrichments = model.enrichments.map((item) => this.enrichmentOf(item, casing));
    const dialects = dialectsOf(model).map(([engine, planned]) => ({
      engine: this.quote(engine),
      ...this.optionsOf({ entity: model.entity, ...planned }),
      schema: schemaPairs({ ...model, ...planned }, spec, casing).map((pair) => ({ from: pair.from, to: pair.to })),
    }));
    const token = model.optimisticConcurrency;
    const vias = new Map<string, string>();
    for (const item of model.enrichments) vias.set(item.via, item.viaType);
    return {
      typeName: model.typeName,
      repoFn: casing.convertFields(`${model.specName}_repository`),
      rowFn: casing.convertFields(`${model.specName}_row_repository`),
      readonlyFn: casing.convertFields(`${model.specName}_readonly_repository`),
      child,
      enrichments,
      wrapped: child.length === 0 ? undefined : {},
      setup: enrichments.length === 0 ? undefined : {},
      builder: dialects.length === 0 ? undefined : { dialects },
      schema: schemaPairs(model, spec, casing).map((pair) => ({ from: pair.from, to: pair.to })),
      createZeros: model.createZeros.map((zero, index) => ({
        name: zero.name,
        boolean: zero.kind === "boolean" ? {} : undefined,
        first: index === 0 ? {} : undefined,
        last: index === model.createZeros.length - 1 ? {} : undefined,
      })),
      readonlyChildren: [...vias].map(([specName]) => ({
        field: child.find((item) => item.specName === specName)?.field,
        module: this.ident(casing.convertFileName(`${specName}_repository`)),
        rowFn: casing.convertFields(`${specName}_row_repository`),
      })),
      ...this.optionsOf(model),
      optimisticConcurrency: token === undefined
        ? undefined
        : { property: token.property, increment: token.increment ? {} : undefined, whereKeys: this.whereKeys(model, token) },
    };
  }

  private registry(models: RepositoryModel[], casing: ICasingStrategy): Record<string, unknown> {
    return {
      models: models.map((model, index) => ({
        typeName: model.typeName,
        entity: this.quote(model.entity),
        plural: this.plural(model.specName, "snakeCase"),
        module: this.ident(casing.convertFileName(`${model.specName}_repository`)),
        repoFn: casing.convertFields(`${model.specName}_repository`),
        readonlyFn: casing.convertFields(`${model.specName}_readonly_repository`),
        optimisticConcurrency: model.optimisticConcurrency === undefined ? undefined : {},
        comma: index > 0 ? {} : undefined,
      })),
    };
  }

  private schemaTypes(models: RepositoryModel[]): Record<string, unknown> {
    const names = new Set<string>();
    for (const model of models) {
      names.add(model.typeName);
      for (const item of model.enrichments) {
        if (item.kind === "list" || item.kind === "linked") names.add(item.viaType);
      }
    }
    return { schemaTypes: [...names].map((name) => ({ name })) };
  }

  private ident(stem: string): string {
    return /^[A-Za-z_][A-Za-z0-9_]*$/.test(stem) ? stem : stem.replaceAll("-", "_");
  }

  private mod(stem: string): Record<string, unknown> {
    const ident = this.ident(stem);
    return { stem, ident, path: ident === stem ? undefined : {} };
  }
}

export const rustRepositories = new RustRepositories();
