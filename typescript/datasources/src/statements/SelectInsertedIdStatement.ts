import type { IStatement } from "./IStatement.ts";

export class SelectInsertedIdStatement implements IStatement {
  constructor(
    readonly table: string,
    readonly column: string,
  ) {}

  getText = (): string => {
    throw new Error("provider must implement SelectInsertIdStatement");
  };
}
