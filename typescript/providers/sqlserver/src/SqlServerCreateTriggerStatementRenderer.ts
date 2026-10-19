import { CreateTriggerStatementTextRenderer, type CreateTriggerStatement } from "@deterministic-code/datasources";

export class SqlServerCreateTriggerStatementRenderer extends CreateTriggerStatementTextRenderer {
  override render(statement: CreateTriggerStatement): string {
    return `CREATE TRIGGER ${statement.name} ON ${statement.table}
AFTER UPDATE AS
BEGIN
  SET NOCOUNT ON;
  UPDATE ${statement.table} SET ${statement.updatedColumn} = ${statement.utcNow}
  FROM ${statement.table} t INNER JOIN inserted i ON t.${statement.keyColumn} = i.${statement.keyColumn};
END`;
  }
}
