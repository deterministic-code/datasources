export type PathParts = { owner: string; last: string; root: boolean };

export const selectFieldAliases = (
  table: string,
  fields: readonly string[],
  aliases: readonly (string | undefined)[] = [],
): (string | undefined)[] => {
  const result: (string | undefined)[] = fields.map(() => undefined);
  const used = new Set<string>();
  for (const [i] of fields.entries()) {
    const alias = aliases[i];
    if (alias !== undefined) {
      result[i] = alias;
      used.add(alias);
    }
  }
  for (const [i, field] of fields.entries()) {
    const path = pathOf(field, table);
    if (result[i] === undefined && path?.root) {
      result[i] = path.last;
      used.add(path.last);
    }
  }
  for (const [i, field] of fields.entries()) {
    const path = pathOf(field, table);
    if (result[i] === undefined && path && !path.root) {
      const alias = used.has(path.last) ? `${path.owner}${path.last}` : path.last;
      result[i] = alias;
      used.add(alias);
    }
  }
  return result;
};

export const applySelectFieldAlias = (field: string, alias?: string): string =>
  alias === undefined ? field : `${field} AS ${alias}`;

export const pathOf = (field: string, table: string): PathParts | undefined => {
  if (asIndexOf(field) >= 0 || field.includes("(")) return undefined;
  const parts = segments(field);
  if (parts.length < 2) return undefined;
  return {
    owner: parts[parts.length - 2],
    last: parts[parts.length - 1],
    root: parts.length === 2 && parts[0] === table,
  };
};

export const functionOf = (field: string): { name: string; inner: string } | undefined => {
  const open = field.indexOf("(");
  return open <= 0 || !field.endsWith(")")
    ? undefined
    : { name: field.slice(0, open).trim(), inner: field.slice(open + 1, -1).trim() };
};

export const asIndexOf = (field: string): number => field.toUpperCase().indexOf(" AS ");

export const segments = (name: string): string[] => {
  const parts: string[] = [];
  let quoted = false;
  let start = 0;
  for (let i = 0; i < name.length; i++) {
    if (name[i] === '"') quoted = !quoted;
    if (name[i] === "." && !quoted) {
      parts.push(name.slice(start, i));
      start = i + 1;
    }
  }
  parts.push(name.slice(start));
  return parts;
};
