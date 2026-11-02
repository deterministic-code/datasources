import { generate as datasource } from "@deterministic-code/rust-generators/generate-datasource-types";
import { generate as views } from "@deterministic-code/rust-generators/generate-view-types";
import { createCasingStrategy, type ICasingStrategy } from "@deterministic-code/generators-common/casing-strategy";
import { authoredViewTypesOf, datasourceTypesOf } from "@deterministic-code/generators-common/spec-types";
import type { IDeterministic } from "@deterministic-code/deterministic-generators-common/parser";
import { fill } from "@deterministic-code/generators-common/fill";
import { content, type GenerateEntry } from "@deterministic-code/generators-common/generate-entry";
import { childClass, dialectsOf, optimisticClass, type RepositoryModel } from "./RepositoryModel.ts";
import { RepositoryGenerator } from "./RepositoryGenerator.ts";
import { schemaPairs } from "./schemaPairs.ts";
import { crateModTmpl, modTmpl, registryTmpl, repositoryTmpl, schemaObjectsTmpl } from "./templates/rust.ts";

class RustRepositories extends RepositoryGenerator {
  readonly types = { datasource, views };

  typesPrefix = (): string => "rust";

  repositoriesPrefix = (casing: ICasingStrategy): string =>
    `rust/${casing.convertDirectories("repositories")}`;

  generate(models: RepositoryModel[], settings: Record<string, string>, spec: IDeterministic): GenerateEntry[] {
    const casing = createCasingStrategy("rust", settings);
    const typeStems = [...new Set([...datasourceTypesOf(spec), ...authoredViewTypesOf(spec)].map((type) => casing.convertFileName(type.name)))].map((stem) => this.mod(stem));
    const mods = [...models.map((model) => this.mod(casing.convertFileName(`${model.specName}_repository`))), this.mod(casing.convertFileName("repository_registry"))];
    return [
      ...models.map((model) => content(`${casing.convertFileName(`${model.specName}_repository`)}.rs`, fill(repositoryTmpl, this.repository(model, spec, casing)))),
      content(`${casing.convertFileName("repository_registry")}.rs`, fill(registryTmpl, this.registry(models, casing, spec))),
      content("mod.rs", fill(modTmpl, { mods, allowNonSnakeCase: this.allowNonSnakeCase(mods) })),
      content("../mod.rs", fill(crateModTmpl, { typeStems, registryIdent: this.ident(casing.convertFileName("repository_registry")), allowNonSnakeCase: this.allowNonSnakeCase(typeStems) })),
      content("../schema_objects.rs", fill(schemaObjectsTmpl, this.schemaTypes(models, spec))),
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
      schema: this.schemaBag(schemaPairs({ ...model, ...planned }, spec, casing)),
    }));
    const token = model.optimisticConcurrency;
    const vias = new Map<string, string>();
    for (const item of model.enrichments) vias.set(item.via, item.viaType);
    const rowEntity = this.isRustEnum(spec, model.specName);
    return {
      typeName: model.typeName,
      entityType: rowEntity ? "datasources::Row" : model.typeName,
      typed: rowEntity ? undefined : {},
      className: childClass(model.typeName),
      optimisticClass: optimisticClass(model.typeName),
      repoFn: casing.convertFields(`${model.specName}_repository`),
      rowFn: casing.convertFields(`${model.specName}_row_repository`),
      readonlyFn: casing.convertFields(`${model.specName}_readonly_repository`),
      child,
      childFields: child.length === 0 ? "" : `, ${child.map((item) => item.field).join(", ")}`,
      enrichments,
      wrapped: child.length === 0 ? undefined : {},
      setup: enrichments.length === 0 ? undefined : {},
      builder: dialects.length === 0 ? undefined : { dialects },
      schema: this.schemaBag(schemaPairs(model, spec, casing)),
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

  private registry(models: RepositoryModel[], casing: ICasingStrategy, spec: IDeterministic): Record<string, unknown> {
    const imports = models.filter((model) => !this.isRustEnum(spec, model.specName));
    return {
      imports: imports.map((model) => ({ typeName: model.typeName })),
      models: models.map((model, index) => ({
        typeName: model.typeName,
        entityType: this.isRustEnum(spec, model.specName) ? "datasources::Row" : model.typeName,
        className: childClass(model.typeName),
        optimisticClass: optimisticClass(model.typeName),
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

  private schemaTypes(models: RepositoryModel[], spec: IDeterministic): Record<string, unknown> {
    const enumNames = new Set(
      spec.types.filter((type) => (type.union ?? []).length > 0 && type.inherits === undefined).map((type) => type.name),
    );
    const names = new Set<string>();
    for (const model of models) {
      if (!enumNames.has(model.specName)) names.add(model.typeName);
      for (const item of model.enrichments) {
        if ((item.kind === "list" || item.kind === "linked") && !enumNames.has(item.via)) names.add(item.viaType);
      }
    }
    return { schemaTypes: [...names].map((name) => ({ name })) };
  }

  private isRustEnum(spec: IDeterministic, specName: string): boolean {
    const type = spec.types.find((item) => item.name === specName);
    return type !== undefined && (type.union ?? []).length > 0 && type.inherits === undefined;
  }

  private ident(stem: string): string {
    return /^[A-Za-z_][A-Za-z0-9_]*$/.test(stem) ? stem : stem.replaceAll("-", "_");
  }

  private allowNonSnakeCase(mods: { ident: string }[]): Record<string, never> | undefined {
    return mods.some((mod) => !/^[a-z][a-z0-9_]*$/.test(mod.ident)) ? {} : undefined;
  }

  private mod(stem: string): Record<string, unknown> {
    const ident = this.ident(stem);
    return { stem, ident, path: ident === stem ? undefined : {} };
  }
}

export const rustRepositories = new RustRepositories();
