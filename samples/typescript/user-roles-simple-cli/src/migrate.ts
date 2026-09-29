import { execute, type ICommand } from "@datasources/core";
import { mysqlProcedures, schema } from "./schema.ts";

export const ifNotExists = (sql: string): string =>
  sql.startsWith("CREATE TABLE ") ? `CREATE TABLE IF NOT EXISTS ${sql.slice("CREATE TABLE ".length)}` : sql;

export const migrate = async (command: ICommand): Promise<void> => {
  const provider = command.connection.provider;
  for (const table of schema(provider))
    await (provider === "sqlite" ? command.execute(ifNotExists(table.getText())) : execute(command, table));
  if (provider === "mysql") for (const sql of mysqlProcedures()) await command.execute(sql);
};
