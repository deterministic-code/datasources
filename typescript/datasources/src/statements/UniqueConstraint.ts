import { constraintNameOf } from "./ConstraintName.ts";
import type { ITableConstraint } from "./ITableConstraint.ts";
import type { StatementNaming } from "./StatementNaming.ts";

export class UniqueConstraint implements ITableConstraint {
  readonly kind = "unique" as const;
  constructor(
    readonly name: string,
    readonly columns: string[],
  ) {}

  static nameOf = (entity: string, naming: StatementNaming, ...parts: string[]): string =>
    constraintNameOf(entity, naming, ...parts);
}
