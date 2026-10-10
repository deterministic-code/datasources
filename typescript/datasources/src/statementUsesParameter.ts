const prefixes = ["$", ":", "@"] as const;

export const statementUsesParameter = (sql: string, name: string): boolean => {
  const bare = name.replace(/^[$:@]/, "");
  return prefixes.some((prefix) => {
    const token = `${prefix}${bare}`;
    let from = 0;
    while (from <= sql.length) {
      const at = sql.indexOf(token, from);
      if (at < 0) return false;
      const next = sql[at + token.length];
      if (next === undefined || !/[0-9A-Za-z_]/.test(next)) return true;
      from = at + token.length;
    }
    return false;
  });
};
