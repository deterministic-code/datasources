import type { ITableConstraint } from "./ITableConstraint.ts";

export class PrimaryKey implements ITableConstraint {
  readonly kind = "primaryKey" as const;
  constructor(
    readonly name: string,
    readonly columns: string[],
  ) {}
}
