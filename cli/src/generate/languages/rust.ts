import { generate as datasource } from "@deterministic-code/rust-generators/generate-datasource-types";
import { generate as views } from "@deterministic-code/rust-generators/generate-view-types";
import type { ICasingStrategy } from "@deterministic-code/generators-common/casing-strategy";
import { rustRepositories } from "../RustRepositories.ts";

export const types = { datasource, views };

export const repositories = rustRepositories;

export const typesPrefix = (): string => "rust";

export const repositoriesPrefix = (casing: ICasingStrategy): string =>
  `rust/${casing.convertDirectories("repositories")}`;
