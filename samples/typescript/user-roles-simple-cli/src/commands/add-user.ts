import type { App } from "../app.ts";
import { flag, ident } from "../schema.ts";
import { writeRow } from "./write-row.ts";

export type AddUser = { kind: "add-user"; username: string; password: string };

export const addUser = async (app: App, parsed: AddUser): Promise<string> => {
  const created = app.stamp();
  const updated = app.token();
  const hashed = app.hash(parsed.password);
  const provider = app.command.connection.provider;
  await writeRow(
    app.command,
    ident(provider, "users"),
    "add_user",
    [
      { name: "username", value: parsed.username },
      { name: "password", value: hashed },
      { name: "is_enabled", value: flag(provider, true) },
      { name: "created", value: created },
      { name: "updated", value: app.bytes(updated) },
    ],
    [
      { name: "p_username", value: parsed.username },
      { name: "p_password", value: hashed },
      { name: "p_is_enabled", value: true },
      { name: "p_created", value: created },
      { name: "p_updated", value: app.bytes(updated) },
    ],
  );
  return "";
};
