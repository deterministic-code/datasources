import type { IStoredProcedureStatement } from "./IStoredProcedureStatement.ts";

export class StoredProcedureStatement implements IStoredProcedureStatement {
  constructor(private readonly text: string) {}
  getText = (): string => this.text;
}
