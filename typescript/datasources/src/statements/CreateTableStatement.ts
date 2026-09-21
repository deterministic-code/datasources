import type { Column } from "./Column.ts";
import type { ICreateTableStatement } from "./ICreateTableStatement.ts";

export class CreateTableStatement implements ICreateTableStatement {
  private readonly text: string;

  constructor(text: string);
  constructor(table: string, columns: Column[]);
  constructor(tableOrText: string, columns?: Column[]) {
    this.text = columns
      ? `CREATE TABLE ${tableOrText} (${columns.map((column) => `${column.name} ${column.type}`).join(", ")})`
      : tableOrText;
  }

  getText = (): string => this.text;
}
