export type TableConstraintKind = "primaryKey" | "foreignKey" | "unique" | "check";

export interface ITableConstraint {
  readonly kind: TableConstraintKind;
  readonly name: string;
}
