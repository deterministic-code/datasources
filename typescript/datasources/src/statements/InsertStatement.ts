import type { IInsertStatement } from "./IInsertStatement.ts";

export class InsertStatement implements IInsertStatement {
  constructor(private readonly text: string) {}
  getText = (): string => this.text;
}
