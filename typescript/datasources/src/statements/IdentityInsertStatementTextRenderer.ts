import type { IdentityInsertStatement } from "./IdentityInsertStatement.ts";
import type { InsertStatement } from "./InsertStatement.ts";

export class IdentityInsertStatementTextRenderer {
  render(statement: IdentityInsertStatement): string {
    return statement.inserts.map((insert) => this.insert(insert)).join("\n");
  }

  protected insert(statement: InsertStatement): string {
    return `INSERT INTO ${statement.table} (${statement.fields.map((field) => field.name).join(", ")}) VALUES (${statement.fields.map((field) => String(field.value)).join(", ")});`;
  }
}
