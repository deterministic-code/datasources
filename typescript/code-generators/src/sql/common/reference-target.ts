import type { TypeField } from "@deterministic-code/deterministic-generators-common/parser";

export const referenceTarget = (
  references: NonNullable<TypeField["references"]>,
): [string, string] => {
  if (Array.isArray(references)) return [references[0], references[1]];
  const [table, col] = references.split(".");
  return [table ?? "", col ?? ""];
};
