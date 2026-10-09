import { execute, Native, StatementTextRenderer, type ICommand } from "@datasources/core";
import { SqliteStatementRenderer } from "@datasources/sqlite";
import { mysqlProcedures, schema } from "./schema.ts";

export const ifNotExists = (sql: string): string =>
  sql.startsWith("CREATE TABLE ") ? `CREATE TABLE IF NOT EXISTS ${sql.slice("CREATE TABLE ".length)}` : sql;

const sqlite = new StatementTextRenderer((text) => new SqliteStatementRenderer(text));

export const migrate = async (command: ICommand): Promise<void> => {
  const provider = command.connection.provider;
  for (const table of schema(provider))
    await (provider === "sqlite"
      ? command.executeNative(new Native(ifNotExists(sqlite.render(table))))
      : execute(command, table));
  if (provider === "mysql") for (const sql of mysqlProcedures()) await command.executeNative(new Native(sql));
};
