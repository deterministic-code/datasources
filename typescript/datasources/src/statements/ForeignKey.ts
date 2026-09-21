import type { ITableConstraint } from "./ITableConstraint.ts";

export const ReferentialAction = {
  Cascade: "cascade",
  SetNull: "setNull",
  SetDefault: "setDefault",
  Restrict: "restrict",
  NoAction: "noAction",
} as const;

export type ReferentialAction = (typeof ReferentialAction)[keyof typeof ReferentialAction];

export class ForeignKey implements ITableConstraint {
  readonly kind = "foreignKey" as const;
  constructor(
    readonly name: string,
    readonly columns: string[],
    readonly referencesTable: string,
    readonly referencesColumns: string[],
    readonly onDelete?: ReferentialAction,
    readonly onUpdate?: ReferentialAction,
  ) {}
}
