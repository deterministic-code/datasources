import {
  CreateIndexStatementTextRenderer,
  DropIndexStatementTextRenderer,
  DropTableStatementTextRenderer,
  DropTriggerStatementTextRenderer,
  type ForeignKeyStatement,
} from "@deterministic-code/datasources";
import { SqliteDriver } from "./SqliteDriver.ts";

export class SqliteDropTableStatementRenderer extends DropTableStatementTextRenderer {
  protected override quote(name: string): string {
    return SqliteDriver.quote(name);
  }
}

export class SqliteCreateIndexStatementRenderer extends CreateIndexStatementTextRenderer {
  protected override quote(name: string): string {
    return SqliteDriver.quote(name);
  }
}

export class SqliteDropIndexStatementRenderer extends DropIndexStatementTextRenderer {
  protected override quote(name: string): string {
    return SqliteDriver.quote(name);
  }
}

export class SqliteDropTriggerStatementRenderer extends DropTriggerStatementTextRenderer {
  protected override quote(name: string): string {
    return SqliteDriver.quote(name);
  }
}

export class SqliteForeignKeyStatementRenderer {
  render(statement: ForeignKeyStatement): string {
    throw new Error(
      statement.kind === "drop" ? "SQLite cannot DROP CONSTRAINT" : "SQLite cannot ADD CONSTRAINT",
    );
  }
}
