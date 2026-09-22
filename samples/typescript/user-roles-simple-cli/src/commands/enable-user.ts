import type { App } from "../app.ts";

export type EnableUser = { kind: "enable-user"; username: string };

export const enableUser = (app: App, parsed: EnableUser): Promise<string> =>
  app.setEnabled(parsed.username, true);
