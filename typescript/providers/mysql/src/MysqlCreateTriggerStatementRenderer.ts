import { CreateTriggerStatementTextRenderer, type CreateTriggerStatement } from "@deterministic-code/datasources";

export class MysqlCreateTriggerStatementRenderer extends CreateTriggerStatementTextRenderer {
  override render(statement: CreateTriggerStatement): string {
    return `CREATE TRIGGER ${statement.name} BEFORE UPDATE ON ${statement.table}
FOR EACH ROW SET NEW.${statement.updatedColumn} = ${statement.utcNow}`;
  }
}
