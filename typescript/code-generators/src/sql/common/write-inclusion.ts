import type { DatasourceFieldOverlay, TypeField } from "@deterministic-code/deterministic-generators-common/parser";

export const resolveWriteInclusion = (
  create: boolean | undefined,
  update: boolean | undefined,
  identity: boolean,
  fixedId: boolean,
  singleGeneratedId: boolean,
): { includeInCreate: boolean; includeInUpdate: boolean } => ({
  includeInCreate: create ?? (!identity || fixedId || !singleGeneratedId),
  includeInUpdate: update ?? (!identity || (!fixedId && !singleGeneratedId)),
});

export const writeInclusionOf = (
  field: TypeField,
  ids: readonly string[],
  overlay?: DatasourceFieldOverlay,
): { includeInCreate: boolean; includeInUpdate: boolean } => {
  const identity = ids.includes(field.name);
  const fixedId = overlay?.isFixedId === true;
  return resolveWriteInclusion(
    overlay?.includeInCreate,
    overlay?.includeInUpdate,
    identity,
    fixedId,
    identity && ids.length === 1 && !fixedId,
  );
};
