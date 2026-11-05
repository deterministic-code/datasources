import {
  CreateIndexStatementTextRenderer,
  CreateTableStatement,
  DropIndexStatementTextRenderer,
  DropTableStatement,
  DropTableStatementTextRenderer,
  DropTriggerStatementTextRenderer,
  type ForeignKeyStatement,
} from "@deterministic-code/datasources";
import { SqliteDriver } from "./SqliteDriver.ts";
import { SqliteCreateTableStatementRenderer } from "./SqliteCreateTableStatementRenderer.ts";

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
    if (statement.kind === "drop") return "";
    if (statement.create === undefined) throw new Error("SQLite cannot ADD CONSTRAINT");
    const tmp = `${statement.table}__new`;
    const rebuilt = new CreateTableStatement(tmp, statement.create.columns, statement.create.constraints);
    const cols = statement.create.columns.map((column) => SqliteDriver.quote(column.name)).join(", ");
    const from = SqliteDriver.quote(statement.table);
    const to = SqliteDriver.quote(tmp);
    return [
      new SqliteCreateTableStatementRenderer().render(rebuilt),
      `INSERT INTO ${to} (${cols}) SELECT ${cols} FROM ${from}`,
      new SqliteDropTableStatementRenderer().render(new DropTableStatement(statement.table)),
      `ALTER TABLE ${to} RENAME TO ${from}`,
    ].join(";\n");
  }
}
