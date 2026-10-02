import type { CreateIndexStatement } from "./CreateIndexStatement.ts";

export class CreateIndexStatementTextRenderer {
  render(statement: CreateIndexStatement): string {
    return `CREATE ${statement.unique ? "UNIQUE " : ""}INDEX ${this.quote(statement.name)} ON ${this.quote(statement.table)} (${statement.columns.map((column) => this.quote(column)).join(", ")})`;
  }

  protected quote(name: string): string {
    return name;
  }
}
