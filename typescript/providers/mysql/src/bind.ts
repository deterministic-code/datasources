import { parametersToRecord, type Cell, type Parameter } from "@deterministic-code/datasources";
import { reverse } from "./converters/MysqlRowTypeConverters.ts";
import type { Scalar } from "./scalar.ts";

export const bindParameters = (
  parameters?: Parameter[] | null,
): Record<string, Scalar> => {
  const record = parametersToRecord(parameters);
  const bound: Record<string, Scalar> = {};
  for (const [name, value] of Object.entries(record))
    bound[name] = reverse(value as Cell) as Scalar;
  return bound;
};
