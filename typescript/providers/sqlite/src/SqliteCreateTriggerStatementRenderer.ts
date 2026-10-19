import { CreateTriggerStatementTextRenderer, type CreateTriggerStatement } from "@deterministic-code/datasources";

export class SqliteCreateTriggerStatementRenderer extends CreateTriggerStatementTextRenderer {
  override render(statement: CreateTriggerStatement): string {
    return `CREATE TRIGGER ${statement.name}
AFTER UPDATE ON ${statement.table}
FOR EACH ROW
WHEN NEW.${statement.updatedColumn} IS OLD.${statement.updatedColumn}
BEGIN
  UPDATE ${statement.table} SET ${statement.updatedColumn} = ${statement.utcNow} WHERE ${statement.keyColumn} = OLD.${statement.keyColumn};
END`;
  }
}
