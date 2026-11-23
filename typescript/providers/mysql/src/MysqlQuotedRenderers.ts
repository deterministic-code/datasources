import {
  CreateIndexStatement,
  CreateIndexStatementTextRenderer,
  CreateTableStatement,
  CreateTriggerStatement,
  DropTableStatement,
  DropTableStatementTextRenderer,
  type ForeignKeyStatement,
  type IStatement,
  type RebuildColumn,
  RebuildTableStatement,
} from "@deterministic-code/datasources";
import { MysqlDriver } from "./MysqlDriver.ts";
import { MysqlCreateTableStatementRenderer } from "./MysqlCreateTableStatementRenderer.ts";
import { MysqlCreateTriggerStatementRenderer } from "./MysqlCreateTriggerStatementRenderer.ts";

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

export class MysqlRebuildTableStatementRenderer {
  render(statement: RebuildTableStatement): string {
    const dest = statement.create.table;
    const tmp = `${dest}__new`;
    const old = `${statement.table}__old`;
    const rebuilt = new CreateTableStatement(tmp, statement.create.columns, statement.create.constraints);
    const names = statement.columns.map((column) => MysqlDriver.quote(column.name)).join(", ");
    const values = statement.columns.map((column) => select(column)).join(", ");
    const from = MysqlDriver.quote(statement.table);
    const to = MysqlDriver.quote(dest);
    const tmpName = MysqlDriver.quote(tmp);
    const oldName = MysqlDriver.quote(old);
    return [
      "SET FOREIGN_KEY_CHECKS = 0",
      new MysqlCreateTableStatementRenderer().render(rebuilt),
      `INSERT INTO ${tmpName} (${names}) SELECT ${values} FROM ${from}`,
      ...statement.recreate.map((item) => recreate(item, tmp)),
      `RENAME TABLE ${from} TO ${oldName}, ${tmpName} TO ${to}`,
      new MysqlDropTableStatementRenderer().render(new DropTableStatement(old)),
      "SET FOREIGN_KEY_CHECKS = 1",
    ]
      .filter((sql) => sql !== "")
      .join(";\n");
  }
}

const select = (column: RebuildColumn): string =>
  column.source !== undefined
    ? column.cast !== undefined
      ? `CAST(${MysqlDriver.quote(column.source)} AS ${castTarget(column.cast)})`
      : MysqlDriver.quote(column.source)
    : (column.literal ?? "NULL");

const castTarget = (type: string): string => {
  const native = type.toUpperCase();
  return native.includes("INT") || native.includes("BOOL")
    ? "SIGNED"
    : native.includes("CHAR") || native.includes("TEXT") || native.includes("UUID")
      ? "CHAR"
      : native.includes("DATE") || native.includes("TIME")
        ? "DATETIME"
        : native.includes("BLOB") || native.includes("BINARY")
          ? "BINARY"
          : native.includes("DEC") || native.includes("NUM") || native.includes("FLOAT") || native.includes("DOUBLE") || native.includes("REAL")
            ? "DECIMAL"
            : "CHAR";
};

const recreate = (statement: IStatement, table: string): string => {
  const index = statement as CreateIndexStatement;
  const trigger = statement as CreateTriggerStatement;
  const handler = {
    CreateIndexStatement: () =>
      new MysqlCreateIndexStatementRenderer().render(
        new CreateIndexStatement(index.name, table, index.columns, index.unique),
      ),
    CreateTriggerStatement: () =>
      new MysqlCreateTriggerStatementRenderer().render(
        new CreateTriggerStatement(table, trigger.name, trigger.updatedColumn, trigger.keyColumn),
      ),
  }[statement.constructor.name];
  if (handler === undefined)
    throw new Error(`${statement.constructor.name} cannot be recreated during a table rebuild`);
  return handler();
};

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
