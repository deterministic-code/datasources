import { ParameterDirection } from "./ParameterDirection.ts";
import type { Parameter } from "./Parameter.ts";
import type { Value } from "./converters/Value.ts";

export class Response {
  readonly result: Value;
  readonly rowsAffected: number;
  readonly parameters?: Parameter[];

  constructor(result: Value, rowsAffected: number, parameters?: Parameter[]) {
    const returned = parameters?.find((parameter) => parameter.direction === ParameterDirection.ReturnValue)?.value;
    this.result = result == null && returned != null ? returned : result;
    this.rowsAffected = rowsAffected;
    if (parameters !== undefined) this.parameters = parameters;
  }
}
