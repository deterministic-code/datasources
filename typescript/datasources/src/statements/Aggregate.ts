const fragment = (op: string, column: string): string => `${op}(${column})`;

export const Aggregate = {
  count: (column: string): string => fragment("COUNT", column),
  sum: (column: string): string => fragment("SUM", column),
  min: (column: string): string => fragment("MIN", column),
  max: (column: string): string => fragment("MAX", column),
};
