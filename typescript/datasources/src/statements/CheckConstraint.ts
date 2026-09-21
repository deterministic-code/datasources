import type { ITableConstraint } from "./ITableConstraint.ts";

export class CheckConstraint implements ITableConstraint {
  readonly kind = "check" as const;
  constructor(
    readonly name: string,
    readonly expression: string,
  ) {}
}
