import { join } from "node:path";
import { overlayEnv, parseEnv, type EnvMap } from "../env.ts";

export const destEnv = async (
  fs: { readFile: (path: string) => Promise<string> },
  cwd: string,
): Promise<EnvMap> => {
  try {
    return overlayEnv(parseEnv(await fs.readFile(join(cwd, ".env"))), process.env);
  } catch {
    return overlayEnv({}, process.env);
  }
};
