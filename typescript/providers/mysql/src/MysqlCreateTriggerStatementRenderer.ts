import { CreateTriggerStatementTextRenderer, type CreateTriggerStatement } from "@deterministic-code/datasources";
import { MysqlDriver } from "./MysqlDriver.ts";

export class MysqlCreateTriggerStatementRenderer extends CreateTriggerStatementTextRenderer {
  override render(statement: CreateTriggerStatement): string {
    return `CREATE TRIGGER ${MysqlDriver.quote(statement.name)} BEFORE UPDATE ON ${MysqlDriver.quote(statement.table)}
FOR EACH ROW SET NEW.${MysqlDriver.quote(statement.updatedColumn)} = ${MysqlDriver.utcNow()}`;
  }
}
