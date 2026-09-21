import type { Field } from "../builders/Field.ts";
import type { IStoredProcedureStatement } from "./IStoredProcedureStatement.ts";

export class StoredProcedureStatement implements IStoredProcedureStatement {
  private readonly text: string;

  constructor(text: string);
  constructor(name: string, parameters: Field[]);
  constructor(nameOrText: string, parameters?: Field[]) {
    this.text = parameters
      ? `CALL ${nameOrText}(${parameters.map((field) => `$${field.name}`).join(", ")})`
      : nameOrText;
  }

  getText = (): string => this.text;
}
