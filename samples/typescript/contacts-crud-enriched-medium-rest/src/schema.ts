import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

export const sqlPath = (start = dirname(fileURLToPath(import.meta.url))): string => {
  for (let dir = start; dir !== dirname(dir); dir = dirname(dir)) {
    const direct = join(dir, "generated", "sql", "sqlite", "migrations", "0001_initial_up.sql");
    if (exists(direct)) return direct;
    const nested = join(
      dir,
      "samples",
      "typescript",
      "contacts-crud-enriched-medium-rest",
      "generated",
      "sql",
      "sqlite",
      "migrations",
      "0001_initial_up.sql",
    );
    if (exists(nested)) return nested;
  }
  throw new Error("generated sql");
};

export const statements = (sql = readFileSync(sqlPath(), "utf8")): string[] => {
  const cleaned = sql
    .split("\n")
    .filter((line) => !line.trimStart().startsWith("--"))
    .join("\n");
  const out: string[] = [];
  let depth = 0;
  let start = 0;
  let dollar = false;
  for (let i = 0; i < cleaned.length; i++) {
    if (cleaned.startsWith("$$", i)) {
      dollar = !dollar;
      i += 1;
      continue;
    }
    if (dollar) continue;
    if (word(cleaned, i, "BEGIN")) depth++;
    if (cleaned[i] !== ";") continue;
    if (endsWithWord(cleaned, start, i, "END") && depth > 0) depth--;
    if (depth !== 0) continue;
    const text = withSeedVersion(cleaned.slice(start, i + 1).trim());
    if (text.length > 0) out.push(text);
    start = i + 1;
  }
  return out;
};

export const withSeedVersion = (sql: string): string => {
  if (!sql.startsWith("INSERT INTO") || sql.includes('"version"') || sql.includes("`version`"))
    return sql;
  const values = sql.indexOf(") VALUES (");
  const quote = sql.includes("`") ? "`" : '"';
  return values < 0
    ? sql
    : `${sql.slice(0, values)}, ${quote}version${quote}) VALUES (${sql.slice(values + ") VALUES (".length, -2)}, X'00');`;
};

const exists = (path: string): boolean => {
  try {
    readFileSync(path);
    return true;
  } catch {
    return false;
  }
};

const word = (sql: string, i: number, token: string): boolean => {
  if (!sql.startsWith(token, i)) return false;
  if (i > 0 && /\w/.test(sql[i - 1]!)) return false;
  const end = i + token.length;
  return end === sql.length || !/\w/.test(sql[end]!);
};

const endsWithWord = (sql: string, start: number, semicolon: number, token: string): boolean => {
  let i = semicolon - 1;
  while (i >= start && /\s/.test(sql[i]!)) i--;
  if (i < start + token.length - 1 || sql.slice(i - token.length + 1, i + 1) !== token) return false;
  const before = i - token.length;
  return before < start || !/\w/.test(sql[before]!);
};
