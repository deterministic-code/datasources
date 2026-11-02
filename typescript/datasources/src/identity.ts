import type { Schema } from "./Schema.ts";

export const idsFromSchema = (schema?: Schema): string[] => schema?.ids() ?? [];

export const identityKey = <T>(value: T, name: string): T => {
  if (value === undefined || value === null || typeof value !== "object") return value;
  const found = (value as Record<string, unknown>)[name];
  return found !== undefined && found !== null && typeof found !== "object" ? (found as T) : value;
};
