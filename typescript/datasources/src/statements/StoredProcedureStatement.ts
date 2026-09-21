import { ParameterDirection } from "../ParameterDirection.ts";
import type { Parameter } from "../Parameter.ts";
import type { IStoredProcedureStatement } from "./IStoredProcedureStatement.ts";

const args = (parameters: Parameter[]): string =>
  parameters
    .filter((parameter) => parameter.direction !== ParameterDirection.ReturnValue)
    .map((parameter) => `$${parameter.name}`)
    .join(", ");

export class StoredProcedureStatement implements IStoredProcedureStatement {
  private readonly text: string;

  constructor(text: string);
  constructor(name: string, parameters: Parameter[]);
  constructor(nameOrText: string, parameters?: Parameter[]) {
    this.text = parameters
      ? `CALL ${nameOrText}(${args(parameters)})`
      : nameOrText;
  }

  getText = (): string => this.text;
}
