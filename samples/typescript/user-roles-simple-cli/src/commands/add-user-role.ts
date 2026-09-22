import type { App } from "../app.ts";
import { columns } from "../columns.ts";
import { writeLink } from "./write-link.ts";

export type AddUserRole = { kind: "add-user-role"; username: string; role: string };

export const addUserRole = (app: App, parsed: AddUserRole): Promise<string> =>
  writeLink(app, parsed.username, "roles", "name", parsed.role, "user_roles", columns.map("role_id"), "add_user_role");
