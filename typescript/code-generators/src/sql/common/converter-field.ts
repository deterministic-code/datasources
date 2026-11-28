import type { TypeField } from "@deterministic-code/deterministic-generators-common/parser";
import type { FieldSize, SpecField } from "@deterministic-code/datasources";
import { referencedSizeOf, type SqlTable } from "./sql-schema.ts";

const specSize = (size: TypeField["size"]): FieldSize | undefined => {
  if (size == null) return size === null ? null : undefined;
  if (size === "unlimited" || typeof size === "number") return size;
  return Array.isArray(size) ? size : undefined;
};

export const converterFieldOf = (
  field: TypeField,
  tables: readonly SqlTable[] = [],
): SpecField => ({
  type: field.type,
  name: field.name,
  size: specSize(field.size) ?? specSize(referencedSizeOf(tables, field)),
  defaultValue: field.defaultValue,
});
