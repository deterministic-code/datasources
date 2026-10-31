import type { GenerateContext } from "@deterministic-code/generators-common/generate-context";
import type { GenerateEntry } from "@deterministic-code/generators-common/generate-entry";
import type { ICasingStrategy } from "@deterministic-code/generators-common/casing-strategy";
import type { IDeterministic } from "@deterministic-code/deterministic-generators-common/parser";
import type { RepositoryModel } from "./RepositoryModel.ts";

export type IRepositoryGenerator = {
  generate(
    models: RepositoryModel[],
    settings: Record<string, string>,
    spec: IDeterministic,
  ): GenerateEntry[];
  types: {
    datasource: (ctx: GenerateContext) => Promise<GenerateEntry[]>;
    views: (ctx: GenerateContext) => Promise<GenerateEntry[]>;
  };
  typesPrefix: (casing: ICasingStrategy, kind: "datasource" | "views") => string;
  repositoriesPrefix: (casing: ICasingStrategy) => string;
};
