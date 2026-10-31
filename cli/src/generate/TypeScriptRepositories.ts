import { createCasingStrategy, type ICasingStrategy } from "@deterministic-code/generators-common/casing-strategy";
import { fill } from "@deterministic-code/generators-common/fill";
import { content, type GenerateEntry } from "@deterministic-code/generators-common/generate-entry";
import { childClass, type RepositoryModel } from "./RepositoryModel.ts";
import { Repositories } from "./Repositories.ts";
import { registryTmpl, repositoryTmpl } from "./templates/typescript.ts";

class TypeScriptRepositories extends Repositories {
  generate(models: RepositoryModel[], settings: Record<string, string>): GenerateEntry[] {
    const casing = createCasingStrategy("typescript", settings);
    return [
      ...models.map((model) => content(`${casing.convertFileName(`${model.specName}_repository`)}.ts`, fill(repositoryTmpl, this.repository(model, casing)))),
      content(`${casing.convertFileName("repository_registry")}.ts`, fill(registryTmpl, this.registry(models, casing))),
    ];
  }

  private repository(model: RepositoryModel, casing: ICasingStrategy): Record<string, unknown> {
    const children = this.childrenOf(model, casing).map((child) => ({
      ...child,
      repoImport: `./${casing.convertFileName(`${child.specName}_repository`)}`,
    }));
    const enrichments = model.enrichments.map((item) => this.enrichmentOf(item, casing));
    const dialects = this.dialectsBag(model);
    const token = model.optimisticConcurrency;
    return {
      typeName: model.typeName,
      typeImport: this.typeImport(model, casing),
      children,
      enrichments,
      setup: children.length === 0 && enrichments.length === 0 ? undefined : {},
      builder: dialects.length === 0 ? undefined : { dialects },
      createZeros: model.createZeros.map((zero, index) => ({
        name: zero.name,
        boolean: zero.kind === "boolean" ? {} : undefined,
        comma: index > 0 ? {} : undefined,
      })),
      ...this.optionsOf(model),
      optimisticConcurrency: token === undefined
        ? undefined
        : { property: token.property, increment: token.increment ? {} : undefined, whereKeys: this.whereKeys(model, token) },
    };
  }

  private registry(models: RepositoryModel[], casing: ICasingStrategy): Record<string, unknown> {
    return {
      imports: models.map((model) => ({
        className: childClass(model.typeName),
        path: `./${casing.convertFileName(`${model.specName}_repository`)}`,
      })),
      factories: models.map((model) => ({
        name: `${model.typeName[0]!.toLowerCase()}${model.typeName.slice(1)}`,
        path: this.typeImport(model, casing),
      })),
      models: models.map((model) => ({
        entity: this.quote(model.entity),
        plural: this.quote(this.plural(model.typeName, "camelCase")),
        pluralName: this.plural(model.typeName, "camelCase"),
        className: childClass(model.typeName),
      })),
    };
  }

  private typeImport(model: RepositoryModel, casing: ICasingStrategy): string {
    const folder = model.typeHome === "view" ? "views" : "datasource";
    const dirs = ["types", "generated", folder].map((segment) => casing.convertDirectories(segment)).join("/");
    return `../${dirs}/${casing.convertFileName(model.specName)}`;
  }
}

export const typescriptRepositories = new TypeScriptRepositories();
