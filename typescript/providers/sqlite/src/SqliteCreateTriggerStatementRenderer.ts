import { CreateTriggerStatementTextRenderer, type CreateTriggerStatement } from "@deterministic-code/datasources";
import { SqliteDriver } from "./SqliteDriver.ts";

export class SqliteCreateTriggerStatementRenderer extends CreateTriggerStatementTextRenderer {
  override render(statement: CreateTriggerStatement): string {
    const table = SqliteDriver.quote(statement.table);
    const name = SqliteDriver.quote(statement.name);
    const updated = SqliteDriver.quote(statement.updatedColumn);
    const key = SqliteDriver.quote(statement.keyColumn);
    return `CREATE TRIGGER ${name}
AFTER UPDATE ON ${table}
FOR EACH ROW
WHEN NEW.${updated} IS OLD.${updated}
BEGIN
  UPDATE ${table} SET ${updated} = ${SqliteDriver.utcNow()} WHERE ${key} = OLD.${key};
END`;
  }
}
