import type { App } from "../app.ts";
import { columns } from "../columns.ts";
import { writeRow } from "./write-row.ts";

export type AddGroup = { kind: "add-group"; name: string };

export const addGroup = async (app: App, parsed: AddGroup): Promise<string> => {
  const column = columns.map("name");
  await writeRow(
    app.command,
    "groups",
    "add_group",
    [
      { name: column, value: parsed.name },
      { name: "is_enabled", value: true },
    ],
    [
      { name: "p_name", value: parsed.name },
      { name: "p_is_enabled", value: true },
    ],
    app.procedures,
  );
  return "";
};
