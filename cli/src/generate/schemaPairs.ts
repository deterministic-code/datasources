import { inferReferenceFieldTypes } from "@deterministic-code/datasources-code-generators/sql/common/infer-reference-types";
import type { ICasingStrategy } from "@deterministic-code/generators-common/casing-strategy";
import { persistedColumnFields } from "@deterministic-code/generators-common/spec-types";
import type { IDeterministic, Type } from "@deterministic-code/deterministic-generators-common/parser";
import type { RepositoryModel } from "./RepositoryModel.ts";

const tableNameOf = (authored: Type, specName: string): string =>
  authored.tags.includes("view_type") &&
  authored.inherits !== undefined &&
  authored.inherits !== "dictionary"
    ? authored.inherits
    : specName;

export const schemaPairs = (
  model: RepositoryModel,
  spec: IDeterministic | undefined,
  casing: ICasingStrategy,
): Array<{ from: string; to: string }> => {
  const expanded = new Map(inferReferenceFieldTypes(spec?.expandedTypes ?? []).map((type) => [type.name, type]));
  const types = new Map(inferReferenceFieldTypes(spec?.types ?? []).map((type) => [type.name, type]));
  const authored = types.get(model.specName) ?? ({ name: model.specName, fields: [], tags: [] } as Type);
  const tableType = (expanded.get(tableNameOf(authored, model.specName)) ?? expanded.get(authored.name) ?? authored) as Type;
  const property = (name: string): string => casing.convertFields(name);
  return persistedColumnFields(tableType, expanded).flatMap((field) =>
    field.type === "string" || field.type.length === 0
      ? []
      : [{ from: property(field.name), to: field.type }],
  );
};
