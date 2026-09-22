import type { App } from "../app.ts";
import { columns } from "../columns.ts";

export type AddGroup = { kind: "add-group"; name: string };

export const addGroup = (app: App, parsed: AddGroup): Promise<string> =>
  app.writeNamed("groups", columns.map("name"), parsed.name, "add_group");
