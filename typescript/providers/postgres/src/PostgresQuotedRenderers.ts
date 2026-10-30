import {
  CreateIndexStatementTextRenderer,
  DropIndexStatementTextRenderer,
  DropTableStatementTextRenderer,
  type ForeignKeyStatement,
} from "@deterministic-code/datasources";
import { PostgresDriver } from "./PostgresDriver.ts";

export class PostgresDropTableStatementRenderer extends DropTableStatementTextRenderer {
  protected override quote(name: string): string {
    return PostgresDriver.quote(name);
  }
}

export class PostgresCreateIndexStatementRenderer extends CreateIndexStatementTextRenderer {
  protected override quote(name: string): string {
    return PostgresDriver.quote(name);
  }
}

export class PostgresDropIndexStatementRenderer extends DropIndexStatementTextRenderer {
  protected override quote(name: string): string {
    return PostgresDriver.quote(name);
  }
}

export class PostgresForeignKeyStatementRenderer {
  render(statement: ForeignKeyStatement): string {
    const table = PostgresDriver.quote(statement.table);
    const name = PostgresDriver.quote(statement.name);
    if (statement.kind === "drop") return `ALTER TABLE ${table} DROP CONSTRAINT ${name}`;
    const cols = statement.columns.map(PostgresDriver.quote).join(", ");
    const refs = statement.referencesColumns.map(PostgresDriver.quote).join(", ");
    return `ALTER TABLE ${table} ADD CONSTRAINT ${name} FOREIGN KEY (${cols}) REFERENCES ${PostgresDriver.quote(statement.referencesTable)} (${refs})`;
  }
}
