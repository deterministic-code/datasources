import { ParameterDirection } from "./ParameterDirection.ts";
import type { Parameter } from "./Parameter.ts";

export class Response {
  readonly result: unknown;
  readonly rowsAffected: number;
  readonly parameters?: Parameter[];

  constructor(result: unknown, rowsAffected: number, parameters?: Parameter[]) {
    const returned = parameters?.find((parameter) => parameter.direction === ParameterDirection.ReturnValue)?.value;
    this.result = result == null && returned != null ? returned : result;
    this.rowsAffected = rowsAffected;
    if (parameters !== undefined) this.parameters = parameters;
  }
}
