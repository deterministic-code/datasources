import { parametersToRecord, type Cell, type Parameter } from "@deterministic-code/datasources";
import { reverse } from "./converters/MysqlRowTypeConverters.ts";
import type { Scalar } from "./scalar.ts";

const NAME = /\$([A-Za-z_][A-Za-z0-9_]*)/g;
const TOKEN = /\$[A-Za-z_][A-Za-z0-9_]*|'[^']*'|"[^"]*"|`[^`]*`|\b[A-Za-z_][A-Za-z0-9_]*\b/g;

export const withoutReturning = (sql: string): string =>
  sql.replace(/\s+RETURNING\b[^;]*$/i, "");

const quoteReserved = (sql: string): string =>
  sql.replace(TOKEN, (token, offset: number) => {
    const skip = token.startsWith("$") || token.startsWith("'") || token.startsWith("\"") || token.startsWith("`");
    const reserved = !skip && token.toLowerCase() === "key"
      && !/\b(?:PRIMARY|FOREIGN|UNIQUE)\s+$/i.test(sql.slice(0, offset));
    const mixed = !skip && /[A-Z]/.test(token) && /[a-z]/.test(token);
    return reserved || mixed ? `\`${token}\`` : token;
  });

export const toMysql = (sql: string): string =>
  quoteReserved(withoutReturning(sql)).replace(NAME, (_match, name: string) => `:${name}`);

export const bindParameters = (
  parameters?: Parameter[] | null,
): Record<string, Scalar> => {
  const record = parametersToRecord(parameters);
  const bound: Record<string, Scalar> = {};
  for (const [name, value] of Object.entries(record))
    bound[name] = reverse(value as Cell) as Scalar;
  return bound;
};
