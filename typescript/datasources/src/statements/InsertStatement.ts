import type { Field } from "../builders/Field.ts";
import type { IInsertStatement } from "./IInsertStatement.ts";

export class InsertStatement implements IInsertStatement {
  readonly table?: string;
  readonly fields: Field[];
  private readonly text?: string;

  constructor(text: string);
  constructor(table: string, fields: Field[]);
  constructor(tableOrText: string, fields?: Field[]) {
    if (fields) {
      this.table = tableOrText;
      this.fields = fields;
      return;
    }
    this.text = tableOrText;
    this.fields = [];
  }

  getText = (): string =>
    this.text ??
    `INSERT INTO ${this.table} (${this.fields.map((field) => field.name).join(", ")}) VALUES (${this.fields.map((field) => `$${field.name}`).join(", ")})`;
}
