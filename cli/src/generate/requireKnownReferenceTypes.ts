import type { IDeterministic, TypeField } from "@deterministic-code/deterministic-generators-common/parser";

const typeNamesOf = (references: TypeField["references"]): string[] => {
  if (references === undefined) return [];
  if (Array.isArray(references)) {
    if (references.every((item) => item.includes(".")))
      return references.map((item) => item.split(".")[0]!).filter((name) => name.length > 0);
    return references[0].length === 0 ? [] : [references[0]];
  }
  const name = references.split(".")[0]!;
  return name.length === 0 ? [] : [name];
};

export const requireKnownReferenceTypes = (spec: IDeterministic): void => {
  const names = new Set([...spec.types, ...spec.expandedTypes].map((type) => type.name));
  for (const type of [...spec.types, ...spec.expandedTypes]) {
    for (const field of type.fields) {
      for (const name of typeNamesOf(field.references)) {
        if (!names.has(name)) throw new Error(`missing type "${name}"`);
      }
    }
  }
};
