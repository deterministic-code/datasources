import {
  CreateIndexStatementTextRenderer,
  DropTableStatementTextRenderer,
  DropTriggerStatementTextRenderer,
  type ForeignKeyStatement,
} from "@deterministic-code/datasources";
import { SqlServerDriver } from "./SqlServerDriver.ts";

export class SqlServerDropTableStatementRenderer extends DropTableStatementTextRenderer {
  protected override quote(name: string): string {
    return SqlServerDriver.quote(name);
  }
}

export class SqlServerCreateIndexStatementRenderer extends CreateIndexStatementTextRenderer {
  protected override quote(name: string): string {
    return SqlServerDriver.quote(name);
  }
}

export class SqlServerDropTriggerStatementRenderer extends DropTriggerStatementTextRenderer {
  protected override quote(name: string): string {
    return SqlServerDriver.quote(name);
  }
}

export class SqlServerForeignKeyStatementRenderer {
  render(statement: ForeignKeyStatement): string {
    const table = SqlServerDriver.quote(statement.table);
    const name = SqlServerDriver.quote(statement.name);
    if (statement.kind === "drop") return `ALTER TABLE ${table} DROP CONSTRAINT ${name}`;
    const cols = statement.columns.map(SqlServerDriver.quote).join(", ");
    const refs = statement.referencesColumns.map(SqlServerDriver.quote).join(", ");
    return `ALTER TABLE ${table} ADD CONSTRAINT ${name} FOREIGN KEY (${cols}) REFERENCES ${SqlServerDriver.quote(statement.referencesTable)} (${refs})`;
  }
}
