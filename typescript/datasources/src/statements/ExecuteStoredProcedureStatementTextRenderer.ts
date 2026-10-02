import { ParameterDirection } from "../ParameterDirection.ts";
import type { Parameter } from "../Parameter.ts";
import type { ExecuteStoredProcedureStatement } from "./ExecuteStoredProcedureStatement.ts";

const args = (parameters: Parameter[]): string =>
  parameters
    .filter((parameter) => parameter.direction !== ParameterDirection.ReturnValue)
    .map((parameter) => `$${parameter.name}`)
    .join(", ");

export class ExecuteStoredProcedureStatementTextRenderer {
  render(statement: ExecuteStoredProcedureStatement): string {
    return `CALL ${statement.name}(${args(statement.parameters)})`;
  }
}
