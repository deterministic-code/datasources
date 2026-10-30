import { DropIndexStatementTextRenderer, type DropIndexStatement } from "@deterministic-code/datasources";
import { MysqlDriver } from "./MysqlDriver.ts";

export class MysqlDropIndexStatementRenderer extends DropIndexStatementTextRenderer {
  override render(statement: DropIndexStatement): string {
    return `DROP INDEX ${MysqlDriver.quote(statement.name)} ON ${MysqlDriver.quote(statement.table)}`;
  }
}
