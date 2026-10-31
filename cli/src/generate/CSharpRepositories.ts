import { createCasingStrategy } from "@deterministic-code/generators-common/casing-strategy";
import { fill } from "@deterministic-code/generators-common/fill";
import { content, type GenerateEntry } from "@deterministic-code/generators-common/generate-entry";
import { childClass, pluralPascal, registryFile, repositoryFile, type RepositoryModel } from "./RepositoryModel.ts";
import { Repositories } from "./Repositories.ts";
import { registryTmpl, repositoryTmpl } from "./templates/csharp.ts";

class CSharpRepositories extends Repositories {
  generate(models: RepositoryModel[], settings: Record<string, string>): GenerateEntry[] {
    const casing = createCasingStrategy("csharp", settings);
    return [
      ...models.map((model) => content(repositoryFile("cs", model.specName, casing), fill(repositoryTmpl, this.repository(model, casing)))),
      content(
        registryFile("cs", casing),
        fill(registryTmpl, {
          registryClass: casing.convertTypes("repository_registry"),
          models: models.map((model) => ({
            entity: this.quote(model.entity),
            plural: this.quote(pluralPascal(model.typeName)),
            typeName: model.typeName,
            className: childClass(model.typeName),
          })),
        }),
      ),
    ];
  }

  private repository(model: RepositoryModel, casing: ReturnType<typeof createCasingStrategy>): Record<string, unknown> {
    const children = this.childrenOf(model, casing);
    const enrichments = model.enrichments.map((item) => this.enrichmentOf(item, casing));
    const dialects = this.dialectsBag(model);
    const token = model.optimisticConcurrency;
    return {
      typeName: model.typeName,
      view: model.typeHome === "view" ? {} : undefined,
      children,
      enrichments,
      setup: children.length === 0 && enrichments.length === 0 ? undefined : {},
      builder: dialects.length === 0 ? undefined : { dialects },
      ...this.optionsOf(model),
      optimisticConcurrency: token === undefined
        ? undefined
        : { property: token.property, increment: token.increment ? {} : undefined, whereKeys: this.whereKeys(model, token) },
    };
  }
}

export const csharpRepositories = new CSharpRepositories();
