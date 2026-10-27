import type { Type, TypeField } from "@deterministic-code/deterministic-generators-common/parser";

const needsType = (field: TypeField): boolean =>
  !field.isArray &&
  !field.isMap &&
  field.references !== undefined &&
  field.type === "string" &&
  field.size === undefined;

const ready = (field: TypeField): boolean =>
  field.type !== "string" || field.size !== undefined;

const targetOf = (references: NonNullable<TypeField["references"]>): [string, string] => {
  if (Array.isArray(references)) return [references[0], references[1]];
  const [table, column] = references.split(".");
  return [table ?? "", column ?? ""];
};

const lookup = (types: Type[], field: TypeField): TypeField | undefined => {
  const [table, column] = targetOf(field.references!);
  return types.find((type) => type.name === table)?.fields.find((item) => item.name === column);
};

export const inferReferenceFieldTypes = (types: Type[]): Type[] => {
  const working = types.map((type) => ({ ...type, fields: type.fields.map((field) => ({ ...field })) }));
  for (let pass = 0; pass < working.length; pass++) {
    let changed = false;
    for (const type of working) {
      type.fields = type.fields.map((field) => {
        if (!needsType(field)) return field;
        const parent = lookup(working, field);
        if (parent === undefined || !ready(parent)) return field;
        changed = true;
        return {
          ...field,
          type: parent.type,
          kind: parent.kind,
          base: parent.base,
          ...(parent.size !== undefined ? { size: parent.size } : {}),
        };
      });
    }
    if (!changed) break;
  }
  return working;
};
