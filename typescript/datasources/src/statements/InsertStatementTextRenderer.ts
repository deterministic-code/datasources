import type { InsertStatement } from "./InsertStatement.ts";

export class InsertStatementTextRenderer {
  render(statement: InsertStatement): string {
    return `INSERT INTO ${this.quote(statement.table)} (${statement.fields.map((field) => this.quote(field.name)).join(", ")}) VALUES (${statement.fields.map((field) => this.parameter(field.name)).join(", ")})`;
  }

  protected quote(name: string): string {
    return name;
  }

  protected parameter(name: string): string {
    return `$${name}`;
  }
}
