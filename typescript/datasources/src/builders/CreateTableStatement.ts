import type { ICreateTableStatement } from "./ICreateTableStatement.ts";

export class CreateTableStatement implements ICreateTableStatement {
  constructor(private readonly text: string) {}
  getText = (): string => this.text;
}
