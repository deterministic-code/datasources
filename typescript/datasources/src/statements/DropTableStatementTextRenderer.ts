import type { DropTableStatement } from "./DropTableStatement.ts";

export class DropTableStatementTextRenderer {
  render(statement: DropTableStatement): string {
    return `DROP TABLE IF EXISTS ${this.quote(statement.table)}`;
  }

  protected quote(name: string): string {
    return name;
  }
}
