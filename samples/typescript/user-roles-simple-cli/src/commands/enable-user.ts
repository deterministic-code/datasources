import type { App } from "../app.ts";
import { setUserEnabled } from "./set-user-enabled.ts";

export type EnableUser = { kind: "enable-user"; username: string };

export const enableUser = (app: App, parsed: EnableUser): Promise<string> =>
  setUserEnabled(app, parsed.username, true);
