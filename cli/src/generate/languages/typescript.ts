import { generate as datasource } from "@deterministic-code/typescript-generators/generate-datasource-types";
import { generate as views } from "@deterministic-code/typescript-generators/generate-view-types";
import type { ICasingStrategy } from "@deterministic-code/generators-common/casing-strategy";
import { typescriptRepositories } from "../TypeScriptRepositories.ts";

export const types = { datasource, views };

export const repositories = typescriptRepositories;

export const typesPrefix = (casing: ICasingStrategy, kind: "datasource" | "views"): string =>
  ["typescript", ...["types", "generated", kind].map((segment) => casing.convertDirectories(segment))].join("/");

export const repositoriesPrefix = (casing: ICasingStrategy): string =>
  `typescript/${casing.convertDirectories("repositories")}`;
