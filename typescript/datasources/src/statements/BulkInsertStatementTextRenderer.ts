import type { BulkInsertStatement } from "./BulkInsertStatement.ts";

export class BulkInsertStatementTextRenderer {
  render = (statement: BulkInsertStatement): string => {
    const values = statement.rows.map(
      (_, index) => `(${statement.columns.map((column) => `$${column}__${index}`).join(", ")})`,
    );
    return `INSERT INTO ${statement.table} (${statement.columns.join(", ")}) VALUES ${values.join(", ")}`;
  };
}
