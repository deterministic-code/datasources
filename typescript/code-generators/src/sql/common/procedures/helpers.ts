/** Helpers shared by stored-procedure generation. */

export interface ProcField {
  name: string;
  type: string;
  size?: number;
  includeInCreate?: boolean;
  includeInUpdate?: boolean;
}

export interface ProcTable {
  fields: ProcField[];
  pkName?: string;
}

/** The entity's primary key: `pkName` when set, else a field named `id`. */
export const pkFieldOf = (table: ProcTable): ProcField =>
  (table.pkName
    ? table.fields.find((f) => f.name === table.pkName)
    : undefined) ??
  table.fields.find((f) => f.name === "id") ?? { name: "id", type: "integer" };

export const createParamFields = (table: ProcTable): ProcField[] =>
  table.fields.filter((f) => f.includeInCreate !== false);

export const updateParamFields = (table: ProcTable): ProcField[] =>
  table.fields.filter((f) => f.includeInUpdate === true);

/** Expanded columns qualified with a table alias (default `t`). */
export const aliasedColumns = (
  table: ProcTable,
  alias = "t",
  columnName: (field: string) => string = (field) => field,
): string =>
  table.fields.map((f) => `${alias}.${columnName(f.name)}`).join(", ");

export const paramAlignWidth = (params: { name: string }[]): number =>
  params.reduce((m, p) => Math.max(m, p.name.length), 0);

export const pad = (s: string, w: number): string =>
  s + " ".repeat(Math.max(0, w - s.length));
