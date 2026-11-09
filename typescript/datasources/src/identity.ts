import type { Schema } from "./Schema.ts";

export const idsFromSchema = (schema?: Schema): string[] =>
  schema === undefined
    ? []
    : Object.entries(schema.columns)
        .filter(([, column]) => column.isId === true)
        .map(([name]) => name);

export const identityKey = <T>(value: T, name: string): T => {
  if (value === undefined || value === null || typeof value !== "object") return value;
  const cells = value as T & Record<string, T>;
  if (!(name in cells)) return value;
  const found = cells[name];
  return found === undefined || found === null ? value : found;
};
