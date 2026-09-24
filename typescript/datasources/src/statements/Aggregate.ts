const fragment = (op: string, column: string, alias?: string): string =>
  alias === undefined ? `${op}(${column})` : `${op}(${column}) AS ${alias}`;

export const Aggregate = {
  count: (column: string, alias?: string): string => fragment("COUNT", column, alias),
  sum: (column: string, alias?: string): string => fragment("SUM", column, alias),
  min: (column: string, alias?: string): string => fragment("MIN", column, alias),
  max: (column: string, alias?: string): string => fragment("MAX", column, alias),
};
