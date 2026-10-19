import { Native, type ICommand } from "@deterministic-code/datasources";
import { statements } from "./schema.ts";

export const migrate = async (command: ICommand): Promise<void> => {
  if (command.connection.provider === "sqlite") await command.executeNative(new Native("PRAGMA foreign_keys = ON"));
  for (const sql of statements()) await command.executeNative(new Native(sql));
};
