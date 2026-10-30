import { constraintNameOf } from "./ConstraintName.ts";
import type { ITableConstraint } from "./ITableConstraint.ts";
import type { StatementNaming } from "./StatementNaming.ts";

export class PrimaryKey implements ITableConstraint {
  readonly kind = "primaryKey" as const;
  constructor(
    readonly name: string,
    readonly columns: string[],
  ) {}

  static nameOf = (entity: string, naming: StatementNaming, ...parts: string[]): string =>
    constraintNameOf(entity, naming, ...parts);
}
