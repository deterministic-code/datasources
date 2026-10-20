import { execute, Native, type ICommand } from "@deterministic-code/datasources";
import { mysqlProcedures, schema } from "./schema.ts";

export const migrate = async (command: ICommand): Promise<void> => {
  const provider = command.connection.provider;
  for (const table of schema(provider)) await execute(command, table);
  if (provider === "mysql") for (const sql of mysqlProcedures()) await command.executeNative(new Native(sql));
};
