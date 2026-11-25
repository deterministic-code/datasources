import { DropIndexStatementTextRenderer, type DropIndexStatement } from "@deterministic-code/datasources";
import { MysqlDriver } from "./MysqlDriver.ts";
import { fitMysqlIdentifier } from "./MysqlIdentifier.ts";

export class MysqlDropIndexStatementRenderer extends DropIndexStatementTextRenderer {
  override render(statement: DropIndexStatement): string {
    return `DROP INDEX ${MysqlDriver.quote(fitMysqlIdentifier(statement.name))} ON ${MysqlDriver.quote(statement.table)}`;
  }
}
