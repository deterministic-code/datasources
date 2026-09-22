import type { App } from "../app.ts";

export type AddRole = { kind: "add-role"; name: string };

export const addRole = (app: App, parsed: AddRole): Promise<string> =>
  app.writeNamed("roles", "name", parsed.name, "add_role");
