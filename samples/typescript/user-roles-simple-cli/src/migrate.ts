import { execute, Native, type ICommand } from "@datasources/core";
import { SqliteDriver } from "@datasources/sqlite";
import { mysqlProcedures, schema } from "./schema.ts";

export const ifNotExists = (sql: string): string =>
  sql.startsWith("CREATE TABLE ") ? `CREATE TABLE IF NOT EXISTS ${sql.slice("CREATE TABLE ".length)}` : sql;

const sqlite = new SqliteDriver().statementRenderer;

export const migrate = async (command: ICommand): Promise<void> => {
  const provider = command.connection.provider;
  for (const table of schema(provider)) {
    try {
      await (provider === "sqlite"
        ? command.executeNative(new Native(ifNotExists(sqlite.render(table))))
        : execute(command, table));
    } catch (error) {
      const text = error instanceof Error ? error.message : String(error);
      if (!/already exists/i.test(text)) throw error;
    }
  }
  if (provider === "mysql") for (const sql of mysqlProcedures()) await command.executeNative(new Native(sql));
};
