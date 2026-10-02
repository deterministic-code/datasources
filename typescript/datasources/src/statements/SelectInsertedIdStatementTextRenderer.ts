import type { SelectInsertedIdStatement } from "./SelectInsertedIdStatement.ts";

export class SelectInsertedIdStatementTextRenderer {
  render(_statement: SelectInsertedIdStatement): string {
    throw new Error("provider must implement SelectInsertIdStatement");
  }
}
