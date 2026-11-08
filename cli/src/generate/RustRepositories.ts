import { generate as datasource } from "@deterministic-code/rust-generators/generate-datasource-types";
import { generate as views } from "@deterministic-code/rust-generators/generate-view-types";
import { createCasingStrategy, type ICasingStrategy } from "@deterministic-code/generators-common/casing-strategy";
import { authoredViewTypesOf, datasourceTypesOf } from "@deterministic-code/generators-common/spec-types";
import type { IDeterministic } from "@deterministic-code/deterministic-generators-common/parser";
import { fill } from "@deterministic-code/generators-common/fill";
import { content, type GenerateEntry } from "@deterministic-code/generators-common/generate-entry";
import type { TypeKind } from "@deterministic-code/generators-common/paths";
import { dialectsOf, isLinked, optimisticClass, type RepositoryModel } from "./RepositoryModel.ts";
import { RepositoryGenerator } from "./RepositoryGenerator.ts";
import { caseDirectories } from "./casing.ts";
import { crateModTmpl, modTmpl, registryTmpl, repositoryTmpl, schemaObjectsTmpl } from "./templates/rust.ts";

class RustRepositories extends RepositoryGenerator {
  readonly types = { datasource, views };

  generate(models: RepositoryModel[], settings: Record<string, string>, spec: IDeterministic): GenerateEntry[] {
    const casing = createCasingStrategy("rust", settings);
    const typeStems = this.typeMods(spec, casing);
    const mods = [...models.map((model) => this.mod(this.repositoryFile(model.specName, casing))), this.mod(casing.convertFileName("repository_registry"))];
    return [
      ...models.map((model) => content(`${this.repositoryFile(model.specName, casing)}.rs`, fill(repositoryTmpl, this.repository(model, spec, casing)))),
      content(`${casing.convertFileName("repository_registry")}.rs`, fill(registryTmpl, this.registry(models, casing, spec))),
      content("mod.rs", fill(modTmpl, { mods, allowNonSnakeCase: this.allowNonSnakeCase(mods) })),
      content("../mod.rs", fill(crateModTmpl, { typeStems, kinds: this.kindMods(typeStems), registryIdent: this.ident(casing.convertFileName("repository_registry")), allowNonSnakeCase: this.allowNonSnakeCase(typeStems) })),
      content("../schema_objects.rs", fill(schemaObjectsTmpl, this.schemaTypes(models, spec))),
    ];
  }

  private repository(model: RepositoryModel, spec: IDeterministic, casing: ICasingStrategy): Record<string, unknown> {
    const child = this.childrenOf(model, casing).map((item, index) => ({
      ...item,
      module: this.ident(this.repositoryFile(item.specName, casing)),
      repoFn: casing.convertFields(this.paths.repositoryStem(item.specName)),
      rowFn: casing.convertFields(this.rowRepositoryStem(item.specName)),
      comma: index > 0 ? {} : undefined,
    }));
    const enrichments = model.enrichments.map((item) => this.enrichmentOf(item, casing));
    const dialects = dialectsOf(model).map(([engine, planned]) => ({
      engine: this.quote(engine),
      ...this.optionsOf({ typeName: model.typeName, ...planned }),
      schema: this.schemaBag(model.schema),
    }));
    const token = model.optimisticConcurrency;
    const vias = new Map<string, string>();
    for (const item of model.enrichments) vias.set(item.via, item.viaType);
    const rowEntity = this.isRustEnum(spec, model.specName);
    return {
      typeName: model.typeName,
      entityType: rowEntity ? "datasources::Row" : model.typeName,
      typed: rowEntity ? undefined : {},
      className: this.repositoryClass(model.specName, casing),
      optimisticClass: optimisticClass(model.typeName),
      repoFn: casing.convertFields(this.paths.repositoryStem(model.specName)),
      rowFn: casing.convertFields(this.rowRepositoryStem(model.specName)),
      readonlyFn: casing.convertFields(this.readonlyRepositoryStem(model.specName)),
      child,
      childFields: child.length === 0 ? "" : `, ${child.map((item) => item.field).join(", ")}`,
      shareConnection: child.length === 0 ? undefined : {},
      enrichments,
      hasEnrichments: enrichments.length === 0 ? undefined : {},
      hasLinked: model.enrichments.some(isLinked) ? {} : undefined,
      setup: enrichments.length === 0 ? undefined : {},
      builder: dialects.length === 0 ? undefined : { dialects },
      schema: this.schemaBag(model.schema),
      readonlyChildren: [...vias].map(([specName]) => ({
        field: child.find((item) => item.specName === specName)?.field,
        module: this.ident(this.repositoryFile(specName, casing)),
        className: this.repositoryClass(specName, casing),
        rowFn: casing.convertFields(this.rowRepositoryStem(specName)),
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
        className: this.repositoryClass(model.specName, casing),
        optimisticClass: optimisticClass(model.typeName),
        plural: this.bindName(model.specName, casing, "fields"),
        module: this.ident(this.repositoryFile(model.specName, casing)),
        repoFn: casing.convertFields(this.paths.repositoryStem(model.specName)),
        readonlyFn: casing.convertFields(this.readonlyRepositoryStem(model.specName)),
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

  private kindMods(typeStems: Record<string, unknown>[]): Record<string, unknown>[] {
    const byKind = new Map<string, { ident: string }[]>();
    for (const item of typeStems) {
      const kind = item.kind;
      if (typeof kind !== "string") continue;
      const idents = byKind.get(kind) ?? [];
      idents.push({ ident: String(item.ident) });
      byKind.set(kind, idents);
    }
    return [...byKind].map(([kind, idents]) => ({ kind, idents }));
  }

  private typeMods(spec: IDeterministic, casing: ICasingStrategy): Record<string, unknown>[] {
    const datasourceNames = new Set(datasourceTypesOf(spec).map((type) => type.name));
    const types = [
      ...datasourceTypesOf(spec).map((type) => ({ name: type.name, kind: "datasource" as TypeKind })),
      ...authoredViewTypesOf(spec)
        .filter((type) => !datasourceNames.has(type.name))
        .map((type) => ({ name: type.name, kind: "view" as TypeKind })),
    ];
    const stems = types.map((type) => ({
      ...type,
      stem: casing.convertFileName(this.paths.typeStem(type.name)),
    }));
    const counts = new Map<string, number>();
    for (const item of stems) counts.set(item.stem, (counts.get(item.stem) ?? 0) + 1);
    return [
      ...new Map(
        stems.map((item) => {
          const collide = counts.get(item.stem)! > 1;
          const ident = this.ident(collide ? `${item.kind}_${item.stem}` : item.stem);
          return [
            `${item.kind}/${item.stem}`,
            {
              stem: item.stem,
              ident,
              path: `${caseDirectories(this.paths.typesDir(item.kind), casing)}/${item.stem}.rs`,
              glob: collide ? undefined : {},
              kind: collide ? item.kind : undefined,
            },
          ];
        }),
      ).values(),
    ];
  }

}

export const rustRepositories = new RustRepositories();
