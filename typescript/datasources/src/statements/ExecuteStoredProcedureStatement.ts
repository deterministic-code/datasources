import { ParameterDirection } from "../ParameterDirection.ts";
import type { Parameter } from "../Parameter.ts";
import type { IExecuteStoredProcedureStatement } from "./IExecuteStoredProcedureStatement.ts";

const args = (parameters: Parameter[]): string =>
  parameters
    .filter((parameter) => parameter.direction !== ParameterDirection.ReturnValue)
    .map((parameter) => `$${parameter.name}`)
    .join(", ");

export class ExecuteStoredProcedureStatement implements IExecuteStoredProcedureStatement {
  readonly name?: string;
  private readonly text: string;

  constructor(text: string);
  constructor(name: string, parameters: Parameter[]);
  constructor(nameOrText: string, parameters?: Parameter[]) {
    this.name = parameters ? nameOrText : undefined;
    this.text = parameters ? `CALL ${nameOrText}(${args(parameters)})` : nameOrText;
  }

  getText = (): string => this.text;
}
