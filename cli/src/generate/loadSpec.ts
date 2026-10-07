import { join } from "node:path";
import type { Fs } from "../io.ts";
import { settingsFrom } from "./settingsFlat.ts";

const specFiles = ["types.yaml", "datasource.yaml", "datasource_seeds.yaml", "services.yaml", "routes.yaml"] as const;

const readOptional = async (fs: Fs, path: string): Promise<string | undefined> => {
  try {
    return await fs.readFile(path);
  } catch {
    return undefined;
  }
};

export const loadSpec = async (
  fs: Fs,
  dir: string,
): Promise<{ files: Record<string, string>; settings: Record<string, string> }> => {
  const files: Record<string, string> = {};
  for (const name of specFiles) {
    const text = await readOptional(fs, join(dir, name));
    if (text !== undefined) files[name] = text;
  }
  if (files["types.yaml"] === undefined) throw new Error(`missing types.yaml in ${dir}`);
  return { files, settings: settingsFrom(await readOptional(fs, join(dir, "settings.yaml"))) };
};
