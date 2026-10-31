import type { Cell } from "./converters/Cell.ts";
import type { SchemaColumn } from "./Schema.ts";

const required: Record<string, Cell> = {
  string: "",
  boolean: false,
};

export const SchemaDefault = {
  of: (column: SchemaColumn): Cell =>
    column.nullable
      ? null
      : column.type.endsWith("integer")
        ? 0
        : (required[column.type] ?? null),
};
