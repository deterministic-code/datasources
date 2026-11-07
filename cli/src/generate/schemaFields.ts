import { inferReferenceFieldTypes } from "@deterministic-code/datasources-code-generators/sql/common/infer-reference-types";
import { persistedColumnFields } from "@deterministic-code/generators-common/spec-types";
import type { IDeterministic, Type } from "@deterministic-code/deterministic-generators-common/parser";

export type SchemaColumn = {
  from: string;
  type: string;
  nullable: boolean;
  hasDefault: boolean;
  isId: boolean;
};

export const inferredSpec = (spec: IDeterministic): IDeterministic => ({
  ...spec,
  types: inferReferenceFieldTypes(spec.types),
  expandedTypes: inferReferenceFieldTypes(spec.expandedTypes),
});

export const schemaFields = (
  type: Type | undefined,
  property: (name: string) => string,
  skip: ReadonlySet<string> = new Set(),
): SchemaColumn[] => {
  if (type === undefined) return [];
  return persistedColumnFields(type).flatMap((field) => {
    const from = property(field.name);
    if (field.type.length === 0 || skip.has(from)) return [];
    return [{
      from,
      type: field.type,
      nullable: field.isNullable === true,
      hasDefault: field.defaultValue !== undefined && field.defaultValue !== null,
      isId: field.isId === true,
    }];
  });
};
