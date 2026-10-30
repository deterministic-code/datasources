import {
  CreateIndexStatementTextRenderer,
  DropTableStatementTextRenderer,
  type ForeignKeyStatement,
} from "@deterministic-code/datasources";
import { MysqlDriver } from "./MysqlDriver.ts";

export class MysqlDropTableStatementRenderer extends DropTableStatementTextRenderer {
  protected override quote(name: string): string {
    return MysqlDriver.quote(name);
  }
}

export class MysqlCreateIndexStatementRenderer extends CreateIndexStatementTextRenderer {
  protected override quote(name: string): string {
    return MysqlDriver.quote(name);
  }
}

export class MysqlForeignKeyStatementRenderer {
  render(statement: ForeignKeyStatement): string {
    const table = MysqlDriver.quote(statement.table);
    const name = MysqlDriver.quote(statement.name);
    if (statement.kind === "drop") return `ALTER TABLE ${table} DROP FOREIGN KEY ${name}`;
    const cols = statement.columns.map(MysqlDriver.quote).join(", ");
    const refs = statement.referencesColumns.map(MysqlDriver.quote).join(", ");
    return `ALTER TABLE ${table} ADD CONSTRAINT ${name} FOREIGN KEY (${cols}) REFERENCES ${MysqlDriver.quote(statement.referencesTable)} (${refs})`;
  }
}
