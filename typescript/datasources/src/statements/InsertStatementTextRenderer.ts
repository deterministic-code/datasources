import type { InsertStatement } from "./InsertStatement.ts";

export class InsertStatementTextRenderer {
  render(statement: InsertStatement): string {
    return `INSERT INTO ${statement.table} (${statement.fields.map((field) => field.name).join(", ")}) VALUES (${statement.fields.map((field) => `$${field.name}`).join(", ")})`;
  }
}
