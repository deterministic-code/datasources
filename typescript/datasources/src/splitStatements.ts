const quoteOf = (char: string): "'" | "\"" | "`" | undefined =>
  char === "'" || char === "\"" || char === "`" ? char : undefined;

export const splitStatements = (sql: string): string[] => {
  const parts: string[] = [];
  let start = 0;
  let quote: "'" | "\"" | "`" | undefined;
  let line = false;
  let block = false;
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
    const opened = quoteOf(char);
    if (opened) {
      quote = opened;
      continue;
    }
    if (char === ";") {
      const part = sql.slice(start, i).trim();
      if (part) parts.push(part);
      start = i + 1;
    }
  }
  const tail = sql.slice(start).trim();
  if (tail) parts.push(tail);
  return parts;
};
