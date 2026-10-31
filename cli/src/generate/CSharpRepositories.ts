import { generate as datasource } from "@deterministic-code/csharp-generators/generate-datasource-types";
import { generate as views } from "@deterministic-code/csharp-generators/generate-view-types";
import { createCasingStrategy, type ICasingStrategy } from "@deterministic-code/generators-common/casing-strategy";
import type { IDeterministic } from "@deterministic-code/deterministic-generators-common/parser";
import { fill } from "@deterministic-code/generators-common/fill";
import { content, type GenerateEntry } from "@deterministic-code/generators-common/generate-entry";
import { childClass, type RepositoryModel } from "./RepositoryModel.ts";
import { RepositoryGenerator } from "./RepositoryGenerator.ts";
import { schemaPairs } from "./schemaPairs.ts";
import { registryTmpl, repositoryTmpl } from "./templates/csharp.ts";

class CSharpRepositories extends RepositoryGenerator {
  readonly types = { datasource, views };

  typesPrefix = (): string => "csharp";

  repositoriesPrefix = (casing: ICasingStrategy): string =>
    `csharp/${casing.convertDirectories("repositories")}`;

  generate(models: RepositoryModel[], settings: Record<string, string>, spec: IDeterministic): GenerateEntry[] {
    const casing = createCasingStrategy("csharp", settings);
    return [
      ...models.map((model) => content(`${casing.convertFileName(`${model.specName}_repository`)}.cs`, fill(repositoryTmpl, this.repository(model, spec, casing)))),
      content(
        `${casing.convertFileName("repository_registry")}.cs`,
        fill(registryTmpl, {
          registryClass: casing.convertTypes("repository_registry"),
          models: models.map((model) => ({
            entity: this.quote(model.entity),
            plural: this.quote(this.plural(model.typeName, "pascalCase")),
            typeName: model.typeName,
            className: childClass(model.typeName),
          })),
        }),
      ),
    ];
  }

  private repository(model: RepositoryModel, spec: IDeterministic, casing: ReturnType<typeof createCasingStrategy>): Record<string, unknown> {
    const children = this.childrenOf(model, casing);
    const enrichments = model.enrichments.map((item) => this.enrichmentOf(item, casing));
    const dialects = this.dialectsBag(model, spec, casing);
    const token = model.optimisticConcurrency;
    return {
      typeName: model.typeName,
      view: model.typeHome === "view" ? {} : undefined,
      children,
      enrichments,
      setup: children.length === 0 && enrichments.length === 0 ? undefined : {},
      builder: dialects.length === 0 ? undefined : { dialects },
      schema: this.schemaBag(schemaPairs(model, spec, casing)),
      ...this.optionsOf(model),
      optimisticConcurrency: token === undefined
        ? undefined
        : { property: token.property, increment: token.increment ? {} : undefined, whereKeys: this.whereKeys(model, token) },
    };
  }
}

export const csharpRepositories = new CSharpRepositories();
