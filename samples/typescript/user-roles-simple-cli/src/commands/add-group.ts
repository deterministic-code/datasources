import type { App } from "../app.ts";
import { columns } from "../columns.ts";
import { flag, ident } from "../schema.ts";
import { writeRow } from "./write-row.ts";

export type AddGroup = { kind: "add-group"; name: string };

export const addGroup = async (app: App, parsed: AddGroup): Promise<string> => {
  const created = app.stamp();
  const updated = app.token();
  const provider = app.command.connection.provider;
  const column = columns.map("name");
  await writeRow(
    app.command,
    ident(provider, "groups"),
    "add_group",
    [
      { name: column, value: parsed.name },
      { name: "is_enabled", value: flag(provider, true) },
      { name: "created", value: created },
      { name: "updated", value: app.bytes(updated) },
    ],
    [
      { name: "p_name", value: parsed.name },
      { name: "p_is_enabled", value: true },
      { name: "p_created", value: created },
      { name: "p_updated", value: app.bytes(updated) },
    ],
  );
  return "";
};
