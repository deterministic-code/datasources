import type { ITableConstraint } from "./ITableConstraint.ts";

export class UniqueConstraint implements ITableConstraint {
  readonly kind = "unique" as const;
  constructor(
    readonly name: string,
    readonly columns: string[],
  ) {}
}
