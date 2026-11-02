import type { App } from "../app.ts";
import { writeRow } from "./write-row.ts";

export type AddUser = { kind: "add-user"; username: string; password: string };

export const addUser = async (app: App, parsed: AddUser): Promise<string> => {
  const hashed = app.hash(parsed.password);
  await writeRow(
    app.command,
    "users",
    "add_user",
    [
      { name: "username", value: parsed.username },
      { name: "password", value: hashed },
      { name: "is_enabled", value: true },
    ],
    [
      { name: "p_username", value: parsed.username },
      { name: "p_password", value: hashed },
      { name: "p_is_enabled", value: true },
    ],
    app.procedures,
  );
  return "";
};
