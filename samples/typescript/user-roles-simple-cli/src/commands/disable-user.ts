import type { App } from "../app.ts";
import { setUserEnabled } from "./set-user-enabled.ts";

export type DisableUser = { kind: "disable-user"; username: string };

export const disableUser = (app: App, parsed: DisableUser): Promise<string> =>
  setUserEnabled(app, parsed.username, false);
