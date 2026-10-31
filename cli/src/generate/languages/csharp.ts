import { generate as datasource } from "@deterministic-code/csharp-generators/generate-datasource-types";
import { generate as views } from "@deterministic-code/csharp-generators/generate-view-types";
import type { ICasingStrategy } from "@deterministic-code/generators-common/casing-strategy";
import { csharpRepositories } from "../CSharpRepositories.ts";

export const types = { datasource, views };

export const repositories = csharpRepositories;

export const typesPrefix = (): string => "csharp";

export const repositoriesPrefix = (casing: ICasingStrategy): string =>
  `csharp/${casing.convertDirectories("repositories")}`;
