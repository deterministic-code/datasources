import type { Field } from "../builders/Field.ts";
import type { IInsertStatement } from "./IInsertStatement.ts";

export class InsertStatement implements IInsertStatement {
  private readonly text: string;

  constructor(text: string);
  constructor(table: string, fields: Field[]);
  constructor(tableOrText: string, fields?: Field[]) {
    this.text = fields
      ? `INSERT INTO ${tableOrText} (${fields.map((field) => field.name).join(", ")}) VALUES (${fields.map((field) => `$${field.name}`).join(", ")})`
      : tableOrText;
  }

  getText = (): string => this.text;
}
