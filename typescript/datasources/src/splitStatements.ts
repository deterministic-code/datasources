const quoteOf = (char: string): "'" | "\"" | "`" | undefined =>
  char === "'" || char === "\"" || char === "`" ? char : undefined;

const word = (sql: string, i: number, token: string): boolean => {
  if (!sql.startsWith(token, i)) return false;
  const before = i === 0 || !/[0-9A-Za-z_]/.test(sql[i - 1]!);
  const after = i + token.length === sql.length || !/[0-9A-Za-z_]/.test(sql[i + token.length]!);
  return before && after;
};

const dollarTag = (sql: string, i: number): string | undefined => {
  if (sql[i] !== "$") return undefined;
  let end = i + 1;
  while (end < sql.length && /[0-9A-Za-z_]/.test(sql[end]!)) end += 1;
  return sql[end] === "$" ? sql.slice(i, end + 1) : undefined;
};

const routine = (sql: string, start: number, i: number): boolean => {
  const head = sql.slice(start, i).toUpperCase();
  return head.includes("CREATE PROCEDURE") || head.includes("CREATE FUNCTION");
};

export const splitStatements = (sql: string): string[] => {
  const parts: string[] = [];
  let start = 0;
  let quote: "'" | "\"" | "`" | undefined;
  let dollar: string | undefined;
  let line = false;
  let block = false;
  let depth = 0;
  for (let i = 0; i < sql.length; i++) {
    const char = sql.charAt(i);
    const next = sql.charAt(i + 1);
    if (line) {
      if (char === "\n") line = false;
      continue;
    }
    if (block) {
      if (char === "*" && next === "/") {
        block = false;
        i += 1;
      }
      continue;
    }
    if (dollar) {
      if (sql.startsWith(dollar, i)) {
        i += dollar.length - 1;
        dollar = undefined;
      }
      continue;
    }
    if (quote) {
      if (char === quote) {
        if (next === quote) i += 1;
        else quote = undefined;
      }
      continue;
    }
    if (char === "-" && next === "-") {
      line = true;
      i += 1;
      continue;
    }
    if (char === "/" && next === "*") {
      block = true;
      i += 1;
      continue;
    }
    const tag = dollarTag(sql, i);
    if (tag) {
      dollar = tag;
      i += tag.length - 1;
      continue;
    }
    const opened = quoteOf(char);
    if (opened) {
      quote = opened;
      continue;
    }
    if (routine(sql, start, i) && word(sql, i, "BEGIN")) {
      depth += 1;
      i += 4;
      continue;
    }
    if (char === ";") {
      if (routine(sql, start, i) && depth > 0) {
        if (/\bEND$/i.test(sql.slice(start, i).trimEnd())) depth -= 1;
        if (depth > 0) continue;
      }
      const part = sql.slice(start, i).trim();
      if (part) parts.push(part);
      start = i + 1;
      depth = 0;
    }
  }
  const tail = sql.slice(start).trim();
  if (tail) parts.push(tail);
  return parts;
};
