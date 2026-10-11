import type { BulkDeleteStatement } from "./BulkDeleteStatement.ts";

export class BulkDeleteStatementTextRenderer {
  render = (statement: BulkDeleteStatement): string => {
    if (statement.ids.length === 0) return `DELETE FROM ${statement.table} WHERE 1 = 1`;
    if (statement.ids.length === 1) {
      const column = statement.ids[0]!;
      const names = statement.rows.map((_, index) => `$${column}__${index}`).join(", ");
      return `DELETE FROM ${statement.table} WHERE ${column} IN (${names})`;
    }
    const tuples = statement.rows
      .map((_, index) => `(${statement.ids.map((column) => `$${column}__${index}`).join(", ")})`)
      .join(", ");
    return `DELETE FROM ${statement.table} WHERE (${statement.ids.join(", ")}) IN (${tuples})`;
  };
}
