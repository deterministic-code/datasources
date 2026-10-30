import { CreateTriggerStatementTextRenderer, type CreateTriggerStatement } from "@deterministic-code/datasources";
import { OracleDriver } from "./OracleDriver.ts";

export class OracleCreateTriggerStatementRenderer extends CreateTriggerStatementTextRenderer {
  override render(statement: CreateTriggerStatement): string {
    return `CREATE OR REPLACE TRIGGER ${OracleDriver.quote(statement.name)}
BEFORE UPDATE ON ${OracleDriver.quote(statement.table)}
FOR EACH ROW
BEGIN
  :NEW.${OracleDriver.quote(statement.updatedColumn)} := ${OracleDriver.utcNow()};
END;
/`;
  }
}
