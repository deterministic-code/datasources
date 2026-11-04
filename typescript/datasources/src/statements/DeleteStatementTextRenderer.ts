import type { DeleteStatement } from "./DeleteStatement.ts";
import type { StatementTextRenderer } from "./StatementTextRenderer.ts";

export class DeleteStatementTextRenderer {
  constructor(private readonly text: StatementTextRenderer) {}

  render(statement: DeleteStatement): string {
    return statement.where
      ? `DELETE FROM ${this.quote(statement.table)} WHERE ${this.text.render(statement.where)}`
      : `DELETE FROM ${this.quote(statement.table)}`;
  }

  protected quote(name: string): string {
    return name;
  }
}
