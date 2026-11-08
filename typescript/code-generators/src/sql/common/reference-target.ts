import type { TypeField } from "@deterministic-code/deterministic-generators-common/parser";

export const referenceTarget = (
  references: NonNullable<TypeField["references"]>,
): [string, string] => {
  if (Array.isArray(references)) {
    if (references.length !== 2) throw new Error(`invalid reference "${references.join(".")}"`);
    return [references[0], references[1]];
  }
  const i = references.indexOf(".");
  if (i < 1 || i === references.length - 1) throw new Error(`invalid reference "${references}"`);
  return [references.slice(0, i), references.slice(i + 1)];
};
