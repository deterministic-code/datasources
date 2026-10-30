import {
  CreateIndexStatementTextRenderer,
  type ForeignKeyStatement,
} from "@deterministic-code/datasources";
import { OracleDriver } from "./OracleDriver.ts";

export class OracleCreateIndexStatementRenderer extends CreateIndexStatementTextRenderer {
  protected override quote(name: string): string {
    return OracleDriver.quote(name);
  }
}

export class OracleForeignKeyStatementRenderer {
  render(statement: ForeignKeyStatement): string {
    const table = OracleDriver.quote(statement.table);
    const name = OracleDriver.quote(statement.name);
    if (statement.kind === "drop") return `ALTER TABLE ${table} DROP CONSTRAINT ${name}`;
    const cols = statement.columns.map(OracleDriver.quote).join(", ");
    const refs = statement.referencesColumns.map(OracleDriver.quote).join(", ");
    return `ALTER TABLE ${table} ADD CONSTRAINT ${name} FOREIGN KEY (${cols}) REFERENCES ${OracleDriver.quote(statement.referencesTable)} (${refs})`;
  }
}
