import type { BulkDeleteStatement } from "./BulkDeleteStatement.ts";

export class BulkDeleteStatementTextRenderer {
  render = (statement: BulkDeleteStatement): string => {
    if (statement.ids.length === 0) return `DELETE FROM ${this.quote(statement.table)} WHERE 1 = 1`;
    if (statement.ids.length === 1) {
      const column = statement.ids[0]!;
      const names = statement.rows.map((_, index) => this.parameter(`${column}__${index}`)).join(", ");
      return `DELETE FROM ${this.quote(statement.table)} WHERE ${this.quote(column)} IN (${names})`;
    }
    const tuples = statement.rows
      .map((_, index) => `(${statement.ids.map((column) => this.parameter(`${column}__${index}`)).join(", ")})`)
      .join(", ");
    return `DELETE FROM ${this.quote(statement.table)} WHERE (${statement.ids.map((column) => this.quote(column)).join(", ")}) IN (${tuples})`;
  };

  protected quote(name: string): string {
    return name;
  }

  protected parameter(name: string): string {
    return `$${name}`;
  }
}
