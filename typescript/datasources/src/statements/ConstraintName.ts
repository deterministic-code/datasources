import type { StatementNaming } from "./StatementNaming.ts";

export const constraintNameOf = (
  entity: string,
  naming: StatementNaming,
  ...parts: string[]
): string =>
  naming.isVerbatim(entity)
    ? [entity, ...parts].join("_")
    : naming.letter(
        naming.convert([naming.physicalStem(entity), ...parts].join("_"), naming.constraintNameCasing),
      );
