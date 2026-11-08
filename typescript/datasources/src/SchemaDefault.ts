import type { Value } from "./converters/Value.ts";
import type { SchemaColumn } from "./Schema.ts";

const required: Record<string, Value> = {
  string: "",
  boolean: false,
};

export const SchemaDefault = {
  of: (column: SchemaColumn): Value =>
    column.nullable
      ? null
      : column.type.endsWith("integer")
        ? 0
        : (required[column.type] ?? null),
};
