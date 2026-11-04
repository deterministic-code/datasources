import type { BulkInsertStatement } from "./BulkInsertStatement.ts";

export class BulkInsertStatementTextRenderer {
  render = (statement: BulkInsertStatement): string => {
    const values = statement.rows.map(
      (_, index) => `(${statement.columns.map((column) => this.parameter(`${column}__${index}`)).join(", ")})`,
    );
    return `INSERT INTO ${this.quote(statement.table)} (${statement.columns.map((column) => this.quote(column)).join(", ")}) VALUES ${values.join(", ")}`;
  };

  protected quote(name: string): string {
    return name;
  }

  protected parameter(name: string): string {
    return `$${name}`;
  }
}
