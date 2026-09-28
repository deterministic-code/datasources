import type { IStatement } from "./IStatement.ts";

export class InsertedIdStatement implements IStatement {
  constructor(
    readonly table: string,
    readonly column: string,
  ) {}

  getText = (): string => `${this.table}.${this.column}`;
}
