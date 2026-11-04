import type { IDeterministic, TypeField } from "@deterministic-code/deterministic-generators-common/parser";

const typeNameOf = (references: TypeField["references"]): string | undefined => {
  if (references === undefined) return undefined;
  if (Array.isArray(references)) return references[0];
  const name = references.split(".")[0]!;
  return name.length === 0 ? undefined : name;
};

export const requireKnownReferenceTypes = (spec: IDeterministic): void => {
  const names = new Set([...spec.types, ...spec.expandedTypes].map((type) => type.name));
  for (const type of [...spec.types, ...spec.expandedTypes]) {
    for (const field of type.fields) {
      const name = typeNameOf(field.references);
      if (name !== undefined && !names.has(name)) throw new Error(`missing type "${name}"`);
    }
  }
};
