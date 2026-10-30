import { DropTriggerStatementTextRenderer, type DropTriggerStatement } from "@deterministic-code/datasources";
import { PostgresDriver } from "./PostgresDriver.ts";

export class PostgresDropTriggerStatementRenderer extends DropTriggerStatementTextRenderer {
  override render(statement: DropTriggerStatement): string {
    return `DROP TRIGGER IF EXISTS ${PostgresDriver.quote(statement.name)} ON ${PostgresDriver.quote(statement.table)}`;
  }
}
