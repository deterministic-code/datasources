import type { App } from "../app.ts";
import { writeRow } from "./write-row.ts";

export type AddRole = { kind: "add-role"; name: string };

export const addRole = async (app: App, parsed: AddRole): Promise<string> => {
  const created = app.stamp();
  const updated = app.token();
  await writeRow(
    app.command,
    "roles",
    "add_role",
    [
      { name: "name", value: parsed.name },
      { name: "is_enabled", value: true },
      { name: "created", value: created },
      { name: "updated", value: app.bytes(updated) },
    ],
    [
      { name: "p_name", value: parsed.name },
      { name: "p_is_enabled", value: true },
      { name: "p_created", value: created },
      { name: "p_updated", value: app.bytes(updated) },
    ],
    app.procedures,
  );
  return "";
};
