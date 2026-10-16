import type { DropIndexStatement } from "./DropIndexStatement.ts";

export class DropIndexStatementTextRenderer {
  render(statement: DropIndexStatement): string {
    return `DROP INDEX IF EXISTS ${this.quote(statement.name)}`;
  }

  protected quote(name: string): string {
    return name;
  }
}
