import type { App } from "../app.ts";

export type DisableUser = { kind: "disable-user"; username: string };

export const disableUser = (app: App, parsed: DisableUser): Promise<string> =>
  app.setEnabled(parsed.username, false);
