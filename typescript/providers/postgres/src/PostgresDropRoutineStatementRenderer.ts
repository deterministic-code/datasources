import { DropRoutineStatementTextRenderer, type DropRoutineStatement } from "@datasources/core";

export class PostgresDropRoutineStatementRenderer extends DropRoutineStatementTextRenderer {
  override render(statement: DropRoutineStatement): string {
    return `DROP FUNCTION IF EXISTS ${statement.name}`;
  }
}
