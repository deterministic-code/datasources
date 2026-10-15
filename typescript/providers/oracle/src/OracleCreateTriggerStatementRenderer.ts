import { CreateTriggerStatementTextRenderer, type CreateTriggerStatement } from "@datasources/core";

export class OracleCreateTriggerStatementRenderer extends CreateTriggerStatementTextRenderer {
  override render(statement: CreateTriggerStatement): string {
    return `CREATE OR REPLACE TRIGGER ${statement.name}
BEFORE UPDATE ON ${statement.table}
FOR EACH ROW
BEGIN
  :NEW.${statement.updatedColumn} := ${statement.utcNow};
END;
/`;
  }
}
