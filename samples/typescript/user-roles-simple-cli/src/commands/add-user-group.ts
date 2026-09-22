import type { App } from "../app.ts";
import { columns } from "../columns.ts";

export type AddUserGroup = { kind: "add-user-group"; username: string; group: string };

export const addUserGroup = (app: App, parsed: AddUserGroup): Promise<string> =>
  app.writeLink(
    parsed.username,
    "groups",
    columns.map("name"),
    parsed.group,
    "user_groups",
    "group_id",
    "add_user_group",
  );
