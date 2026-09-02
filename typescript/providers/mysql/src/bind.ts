import { parametersToRecord, type Parameter } from "@datasources/core";
import type { Scalar } from "./scalar.ts";

const NAME = /\$([A-Za-z_][A-Za-z0-9_]*)/g;

export const withoutReturning = (sql: string): string =>
  sql.replace(/\s+RETURNING\b[^;]*$/i, "");

export const toMysql = (sql: string): string =>
  withoutReturning(sql).replace(NAME, (_match, name: string) => `:${name}`);

export const bindParameters = (
  parameters?: Parameter[] | null,
): Record<string, Scalar> => {
  const record = parametersToRecord(parameters);
  const bound: Record<string, Scalar> = {};
  for (const [name, value] of Object.entries(record))
    bound[name] = value as Scalar; // Parameter.value is a Scalar at the library boundary
  return bound;
};
