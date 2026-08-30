import type { Cell } from "./Cell.ts";
import type { FieldConverter } from "./FieldConverter.ts";
import type { ProviderTypes } from "./ProviderTypes.ts";
import { binaryFieldConverter } from "./binaryFieldConverter.ts";
import { booleanFieldConverter } from "./booleanFieldConverter.ts";
import { dateTimeFieldConverter } from "./dateTimeFieldConverter.ts";
import { decimalFieldConverter } from "./decimalFieldConverter.ts";
import { identityFieldConverter } from "./identityFieldConverter.ts";
import { uuidFieldConverter } from "./uuidFieldConverter.ts";

export const canonicalConverters = (
  booleanStorage: "integer" | "native" = "integer",
): Record<string, FieldConverter> => ({
  TEXT: identityFieldConverter,
  INTEGER: identityFieldConverter,
  REAL: identityFieldConverter,
  NUMERIC: decimalFieldConverter(),
  BOOLEAN: booleanFieldConverter(booleanStorage),
  DATETIME: dateTimeFieldConverter,
  BLOB: binaryFieldConverter,
  UUID: uuidFieldConverter,
});

export { identityFieldConverter };

export const sqlProviderTypes = (
  aliases: Record<string, string> = {},
  options: { rename?: boolean; booleanStorage?: "integer" | "native" } = {},
): ProviderTypes => {
  const types = canonicalConverters(options.booleanStorage ?? "integer");
  for (const [alias, target] of Object.entries(aliases))
    types[alias] = types[target] ?? identityFieldConverter;
  return { rename: options.rename !== false, types };
};

export const identityProviderTypes = (): ProviderTypes => ({
  rename: false,
  types: {},
});

export const inferSqlType = (value: Cell | object): string => {
  if (value instanceof Date) return "DATETIME";
  if (value instanceof Uint8Array) return "BLOB";
  if (typeof value === "boolean") return "BOOLEAN";
  if (typeof value === "bigint") return "INTEGER";
  if (typeof value === "number")
    return Number.isInteger(value) ? "INTEGER" : "REAL";
  return "TEXT";
};

export const normalizeSqlType = (sqlType: string): string => {
  const trimmed = sqlType.trim().toUpperCase();
  if (trimmed.startsWith("TINYINT(1)")) return "TINYINT(1)";
  if (trimmed.startsWith("CHAR(36)")) return "CHAR(36)";
  const paren = trimmed.indexOf("(");
  return paren === -1 ? trimmed : trimmed.slice(0, paren).trim();
};
