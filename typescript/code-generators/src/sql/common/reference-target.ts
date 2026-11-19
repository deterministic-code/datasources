import type { TypeField } from "@deterministic-code/deterministic-generators-common/parser";

export type ReferencePair = { type: string; field: string };

const split = (value: string): ReferencePair | undefined => {
  const i = value.indexOf(".");
  if (i < 1 || i === value.length - 1) return undefined;
  return { type: value.slice(0, i), field: value.slice(i + 1) };
};

/** A dotted array is one `type.field` per column. A dotless array is `[type, field]`. */
export const referencePairs = (references: TypeField["references"]): ReferencePair[] => {
  if (references === undefined) return [];
  if (Array.isArray(references)) {
    if (references.every((item) => item.includes("."))) return references.flatMap((item) => {
      const pair = split(item);
      return pair === undefined ? [] : [pair];
    });
    const [type, field] = references;
    return type !== undefined && field !== undefined && type.length > 0 && field.length > 0 ? [{ type, field }] : [];
  }
  const pair = split(references);
  return pair === undefined ? [] : [pair];
};

export const referenceTarget = (
  references: NonNullable<TypeField["references"]>,
): [string, string] => {
  if (Array.isArray(references)) {
    if (references.length !== 2) throw new Error(`invalid reference "${references.join(".")}"`);
    if (references.every((item) => item.includes(".")))
      throw new Error(`composite reference "${references.join(", ")}" is not a single column`);
    return [references[0], references[1]];
  }
  const i = references.indexOf(".");
  if (i < 1 || i === references.length - 1) throw new Error(`invalid reference "${references}"`);
  return [references.slice(0, i), references.slice(i + 1)];
};
