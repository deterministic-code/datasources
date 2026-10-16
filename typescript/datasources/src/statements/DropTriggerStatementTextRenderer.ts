import type { DropTriggerStatement } from "./DropTriggerStatement.ts";

export class DropTriggerStatementTextRenderer {
  render(statement: DropTriggerStatement): string {
    return `DROP TRIGGER IF EXISTS ${this.quote(statement.name)}`;
  }

  protected quote(name: string): string {
    return name;
  }
}
