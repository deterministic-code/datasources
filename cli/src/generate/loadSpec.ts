import { join } from "node:path";
import type { Fs } from "../io.ts";
import { settingsFrom } from "./settingsFlat.ts";

const specFiles = ["types.yaml", "datasource.yaml", "datasource_seeds.yaml", "services.yaml", "routes.yaml"] as const;

export const defaultDatasourceYaml = `version: 1.0.0
includes:
  - types:
      filter: tag == "datasource_type" || tag == "view_type"
types: []
`;

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
  files["datasource.yaml"] ??= defaultDatasourceYaml;
  return { files, settings: settingsFrom(await readOptional(fs, join(dir, "settings.yaml"))) };
};
