import {
  CreateIndexStatement,
  CreateIndexStatementTextRenderer,
  CreateTableStatement,
  CreateTriggerStatement,
  DropIndexStatementTextRenderer,
  DropTableStatement,
  DropTableStatementTextRenderer,
  DropTriggerStatementTextRenderer,
  type ForeignKeyStatement,
  type IStatement,
  RebuildColumn,
  RebuildTableStatement,
} from "@deterministic-code/datasources";
import { SqliteDriver } from "./SqliteDriver.ts";
import { SqliteCreateTableStatementRenderer } from "./SqliteCreateTableStatementRenderer.ts";
import { SqliteCreateTriggerStatementRenderer } from "./SqliteCreateTriggerStatementRenderer.ts";

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
    return new SqliteRebuildTableStatementRenderer().render(
      new RebuildTableStatement(
        statement.table,
        statement.create,
        statement.create.columns.map((column) => new RebuildColumn(column.name, column.name)),
      ),
    );
  }
}

export class SqliteRebuildTableStatementRenderer {
  render(statement: RebuildTableStatement): string {
    const dest = statement.create.table;
    const tmp = `${dest}__new`;
    const rebuilt = new CreateTableStatement(tmp, statement.create.columns, statement.create.constraints);
    const names = statement.columns.map((column) => SqliteDriver.quote(column.name)).join(", ");
    const values = statement.columns.map((column) => select(column)).join(", ");
    const from = SqliteDriver.quote(statement.table);
    const to = SqliteDriver.quote(dest);
    const tmpName = SqliteDriver.quote(tmp);
    return [
      new SqliteCreateTableStatementRenderer().render(rebuilt),
      `INSERT INTO ${tmpName} (${names}) SELECT ${values} FROM ${from}`,
      new SqliteDropTableStatementRenderer().render(new DropTableStatement(statement.table)),
      `ALTER TABLE ${tmpName} RENAME TO ${to}`,
      ...statement.recreate.map((item) => recreate(item)),
    ]
      .filter((sql) => sql !== "")
      .join(";\n");
  }
}

const select = (column: RebuildColumn): string =>
  column.source !== undefined
    ? column.cast !== undefined
      ? `CAST(${SqliteDriver.quote(column.source)} AS ${column.cast})`
      : SqliteDriver.quote(column.source)
    : (column.literal ?? "NULL");

const recreate = (statement: IStatement): string => {
  const index = statement as CreateIndexStatement;
  const trigger = statement as CreateTriggerStatement;
  const handler = {
    CreateIndexStatement: () =>
      new SqliteCreateIndexStatementRenderer().render(
        new CreateIndexStatement(index.name, index.table, index.columns, index.unique),
      ),
    CreateTriggerStatement: () =>
      new SqliteCreateTriggerStatementRenderer().render(
        new CreateTriggerStatement(trigger.table, trigger.name, trigger.updatedColumn, trigger.keyColumn),
      ),
  }[statement.constructor.name];
  if (handler === undefined)
    throw new Error(`${statement.constructor.name} cannot be recreated during a table rebuild`);
  return handler();
};
