import { generate as datasource } from "@deterministic-code/typescript-generators/generate-datasource-types";
import { generate as views } from "@deterministic-code/typescript-generators/generate-view-types";
import { createCasingStrategy, type ICasingStrategy } from "@deterministic-code/generators-common/casing-strategy";
import type { IDeterministic } from "@deterministic-code/deterministic-generators-common/parser";
import { fill } from "@deterministic-code/generators-common/fill";
import { content, type GenerateEntry } from "@deterministic-code/generators-common/generate-entry";
import type { TypeKind } from "@deterministic-code/generators-common/paths";
import { caseDirectories } from "./casing.ts";
import type { RepositoryModel } from "./RepositoryModel.ts";
import { RepositoryGenerator } from "./RepositoryGenerator.ts";
import { registryTmpl, repositoryTmpl } from "./templates/typescript.ts";

class TypeScriptRepositories extends RepositoryGenerator {
  readonly types = { datasource, views };

  generate(models: RepositoryModel[], settings: Record<string, string>, _spec: IDeterministic): GenerateEntry[] {
    const casing = createCasingStrategy("typescript", settings);
    return [
      ...models.map((model) => content(`${this.repositoryFile(model.specName, casing)}.ts`, fill(repositoryTmpl, this.repository(model, casing)))),
      content(`${casing.convertFileName("repository_registry")}.ts`, fill(registryTmpl, this.registry(models, casing))),
    ];
  }

  private repository(model: RepositoryModel, casing: ICasingStrategy): Record<string, unknown> {
    const children = this.childrenOf(model, casing).map((child) => ({
      ...child,
      repoImport: `./${this.repositoryFile(child.specName, casing)}`,
    }));
    const enrichments = model.enrichments.map((item) => this.enrichmentOf(item, casing));
    const dialects = this.dialectsBag(model, casing);
    const token = model.optimisticConcurrency;
    return {
      typeName: model.typeName,
      className: this.repositoryClass(model.specName, casing),
      typeImport: this.typeImport(model, casing),
      children,
      enrichments,
      setup: children.length === 0 && enrichments.length === 0 ? undefined : {},
      builder: dialects.length === 0 ? undefined : { dialects },
      schema: this.schemaBag(model.schema, casing),
      ...this.optionsOf(model),
      optimisticConcurrency: token === undefined
        ? undefined
        : { property: token.property, increment: token.increment ? {} : undefined, whereKeys: this.whereKeys(model, token) },
    };
  }

  private registry(models: RepositoryModel[], casing: ICasingStrategy): Record<string, unknown> {
    return {
      imports: models.map((model) => ({
        className: this.repositoryClass(model.specName, casing),
        path: `./${this.repositoryFile(model.specName, casing)}`,
      })),
      factories: models.map((model) => ({
        name: casing.convertFields(this.paths.typeStem(model.specName)),
        path: this.typeImport(model, casing),
      })),
      models: models.map((model) => ({
        typeName: this.quote(model.typeName),
        plural: this.quote(this.bindName(model.specName, casing, "fields")),
        pluralName: this.bindName(model.specName, casing, "fields"),
        className: this.repositoryClass(model.specName, casing),
      })),
    };
  }

  private typeImport(model: RepositoryModel, casing: ICasingStrategy): string {
    const kind: TypeKind = model.typeHome === "view" ? "view" : "datasource";
    return `../${caseDirectories(this.paths.typesDir(kind), casing)}/${casing.convertFileName(this.paths.typeStem(model.specName))}`;
  }
}

export const typescriptRepositories = new TypeScriptRepositories();
