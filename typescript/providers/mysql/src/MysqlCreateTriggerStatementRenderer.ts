import { CreateTriggerStatementTextRenderer, type CreateTriggerStatement } from "@deterministic-code/datasources";
import { MysqlDriver } from "./MysqlDriver.ts";
import { fitMysqlIdentifier } from "./MysqlIdentifier.ts";

export class MysqlCreateTriggerStatementRenderer extends CreateTriggerStatementTextRenderer {
  override render(statement: CreateTriggerStatement): string {
    return `CREATE TRIGGER ${MysqlDriver.quote(fitMysqlIdentifier(statement.name))} BEFORE UPDATE ON ${MysqlDriver.quote(statement.table)}
FOR EACH ROW SET NEW.${MysqlDriver.quote(statement.updatedColumn)} = ${MysqlDriver.utcNow()}`;
  }
}

export const dropTriggerSql = (name: string): string =>
  `DROP TRIGGER IF EXISTS ${MysqlDriver.quote(fitMysqlIdentifier(name))}`;
