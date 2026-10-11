export const statementParameters = (sql: string): string[] => {
  const names: string[] = [];
  const seen = new Set<string>();
  for (let i = 0; i < sql.length; i++) {
    const char = sql[i];
    if (char !== "$" && char !== ":" && char !== "@") continue;
    let end = i + 1;
    while (end < sql.length && /[0-9A-Za-z_]/.test(sql[end]!)) end += 1;
    if (end === i + 1) continue;
    if (char === "$" && sql[end] === "$") {
      i = end;
      continue;
    }
    const name = sql.slice(i + 1, end);
    if (!seen.has(name)) {
      seen.add(name);
      names.push(name);
    }
    i = end - 1;
  }
  return names;
};
