import type { ICasingStrategy } from "@deterministic-code/generators-common/casing-strategy";
import { persistedColumnFields } from "@deterministic-code/generators-common/spec-types";
import type { IDeterministic, Type } from "@deterministic-code/deterministic-generators-common/parser";
import type { RepositoryModel } from "./RepositoryModel.ts";

export const schemaPairs = (
  model: RepositoryModel,
  spec: IDeterministic | undefined,
  casing: ICasingStrategy,
): Array<{ from: string; to: string }> => {
  const expanded = new Map((spec?.expandedTypes ?? []).map((type) => [type.name, type]));
  const types = new Map((spec?.types ?? []).map((type) => [type.name, type]));
  const authored = types.get(model.specName) ?? { name: model.specName, fields: [] };
  const tableType = (expanded.get(authored.name) ?? authored) as Type;
  const property = (name: string): string => casing.convertFields(name);
  return persistedColumnFields(tableType, expanded).flatMap((field) =>
    field.type === "string" ? [] : [{ from: property(field.name), to: field.type }],
  );
};
