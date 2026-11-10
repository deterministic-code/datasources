import { generate as datasource } from "@deterministic-code/csharp-generators/generate-datasource-types";
import { generate as views } from "@deterministic-code/csharp-generators/generate-view-types";
import { createCasingStrategy, type ICasingStrategy } from "@deterministic-code/generators-common/casing-strategy";
import type { IDeterministic } from "@deterministic-code/deterministic-generators-common/parser";
import { fill } from "@deterministic-code/generators-common/fill";
import { content, type GenerateEntry } from "@deterministic-code/generators-common/generate-entry";
import type { RepositoryModel } from "./RepositoryModel.ts";
import { RepositoryGenerator } from "./RepositoryGenerator.ts";
import { registryTmpl, repositoryTmpl } from "./templates/csharp.ts";

class CSharpRepositoryGenerators extends RepositoryGenerator {
  readonly types = { datasource, views };

  typesPrefix = (): string => "";

  generate(models: RepositoryModel[], settings: Record<string, string>, _spec: IDeterministic): GenerateEntry[] {
    const casing = createCasingStrategy("csharp", settings);
    return [
      ...models.map((model) => content(`${this.repositoryFile(model.specName, casing)}.cs`, fill(repositoryTmpl, this.repository(model, casing)))),
      content(
        `${casing.convertFileName("repository_registry")}.cs`,
        fill(registryTmpl, {
          registryClass: casing.convertTypes("repository_registry"),
          models: models.map((model) => ({
            plural: this.quote(this.bindName(model.specName, casing, "types")),
            typeName: model.typeName,
            className: this.repositoryClass(model.specName, casing),
          })),
        }),
      ),
    ];
  }

  private repository(model: RepositoryModel, casing: ICasingStrategy): Record<string, unknown> {
    const children = this.childrenOf(model, casing);
    const enrichments = model.enrichments.map((item) => this.enrichmentOf(item, casing));
    const dialects = this.dialectsBag(model);
    const token = model.optimisticConcurrency;
    return {
      typeName: model.typeName,
      className: this.repositoryClass(model.specName, casing),
      view: model.typeHome === "view" ? {} : undefined,
      readonly: model.readonly ? {} : undefined,
      children,
      enrichments,
      setup: children.length === 0 && enrichments.length === 0 ? undefined : {},
      builder: dialects.length === 0 ? undefined : { dialects },
      schema: this.schemaBag(model.schema),
      ...this.optionsOf(model),
      optimisticConcurrency: token === undefined
        ? undefined
        : { property: token.property, increment: token.increment ? {} : undefined, whereKeys: this.whereKeys(model, token) },
    };
  }
}

export const csharpRepositoryGenerators = new CSharpRepositoryGenerators();
