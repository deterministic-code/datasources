import type { App } from "../app.ts";
import { writeRow } from "./write-row.ts";

export type AddRole = { kind: "add-role"; name: string };

export const addRole = async (app: App, parsed: AddRole): Promise<string> => {
  await writeRow(
    app.command,
    "roles",
    "add_role",
    [
      { name: "name", value: parsed.name },
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
