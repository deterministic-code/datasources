import { parametersToRecord, type Cell, type Parameter } from "@deterministic-code/datasources";
import { isBase64 } from "./converters/BinaryConverter.ts";
import { isIsoDate } from "./converters/DateTimeConverter.ts";
import { reverse } from "./converters/PostgresRowTypeConverters.ts";
import type { Scalar, Wire } from "./scalar.ts";

const NAME = /\$([A-Za-z_][A-Za-z0-9_]*)/g;

const quoteMixedCase = (sql: string): string =>
  sql.replace(/\$[A-Za-z_][A-Za-z0-9_]*|'[^']*'|"[^"]*"|`[^`]*`|\b[A-Za-z_][A-Za-z0-9_]*\b/g, (token) =>
    token.startsWith("$") || token.startsWith("'") || token.startsWith("\"") || token.startsWith("`")
      || !/[A-Z]/.test(token) || !/[a-z]/.test(token)
      ? token
      : `"${token}"`,
  );

const wire = (value: Scalar): Wire =>
  typeof value !== "string"
    ? value
    : isIsoDate(value)
      ? new Date(value)
      : isBase64(value)
        ? Uint8Array.from(Buffer.from(value, "base64"))
        : value;

export const toPostgres = (
  sql: string,
  params?: Record<string, Scalar>,
): { text: string; values: Wire[] } => {
  const names: string[] = [];
  const text = quoteMixedCase(sql).replace(NAME, (_match, name: string) => {
    let index = names.indexOf(name);
    if (index < 0) {
      names.push(name);
      index = names.length - 1;
    }
    return `$${index + 1}`;
  });
  return { text, values: names.map((name) => wire(params?.[name] ?? null)) };
};

export const bindParameters = (
  parameters?: Parameter[] | null,
): Record<string, Scalar> => {
  const record = parametersToRecord(parameters);
  const bound: Record<string, Scalar> = {};
  for (const [name, value] of Object.entries(record))
    bound[name] = reverse(value as Cell) as Scalar;
  return bound;
};
