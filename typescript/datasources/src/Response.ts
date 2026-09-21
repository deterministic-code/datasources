import { ParameterDirection } from "./ParameterDirection.ts";
import type { Parameter } from "./Parameter.ts";

export interface Response {
  result: unknown;
  rowsAffected: number;
  parameters?: Parameter[];
}

export const responseOf = (
  result: unknown,
  rowsAffected: number,
  parameters?: Parameter[],
): Response => {
  const ret = parameters?.find((p) => p.direction === ParameterDirection.ReturnValue)?.value;
  const resolved = result == null && ret != null ? ret : result;
  return parameters === undefined
    ? { result: resolved, rowsAffected }
    : { result: resolved, rowsAffected, parameters };
};
