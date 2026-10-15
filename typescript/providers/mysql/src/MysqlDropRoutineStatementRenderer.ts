import { DropRoutineStatementTextRenderer, type DropRoutineStatement } from "@datasources/core";

export class MysqlDropRoutineStatementRenderer extends DropRoutineStatementTextRenderer {
  override render(statement: DropRoutineStatement): string {
    return `DROP PROCEDURE IF EXISTS ${statement.name}`;
  }
}
