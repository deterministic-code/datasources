import type { App } from "../app.ts";
import { columns } from "../columns.ts";
import { writeLink } from "./write-link.ts";

export type AddUserGroup = { kind: "add-user-group"; username: string; group: string };

export const addUserGroup = (app: App, parsed: AddUserGroup): Promise<string> =>
  writeLink(app, parsed.username, "groups", columns.map("name"), parsed.group, "user_groups", "group_id", "add_user_group");
