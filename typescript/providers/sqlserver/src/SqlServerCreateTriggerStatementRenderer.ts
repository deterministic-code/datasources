import { CreateTriggerStatementTextRenderer, type CreateTriggerStatement } from "@deterministic-code/datasources";
import { SqlServerDriver } from "./SqlServerDriver.ts";

export class SqlServerCreateTriggerStatementRenderer extends CreateTriggerStatementTextRenderer {
  override render(statement: CreateTriggerStatement): string {
    const table = SqlServerDriver.quote(statement.table);
    const updated = SqlServerDriver.quote(statement.updatedColumn);
    const key = SqlServerDriver.quote(statement.keyColumn);
    return `CREATE TRIGGER ${SqlServerDriver.quote(statement.name)} ON ${table}
AFTER UPDATE AS
BEGIN
  SET NOCOUNT ON;
  UPDATE ${table} SET ${updated} = ${SqlServerDriver.utcNow()}
  FROM ${table} t INNER JOIN inserted i ON t.${key} = i.${key};
END`;
  }
}
