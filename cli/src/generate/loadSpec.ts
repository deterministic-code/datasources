import { join } from "node:path";
import type { Fs } from "../io.ts";
import { settingsFrom } from "./settingsFlat.ts";
import { resolveIncludes } from "./resolveIncludes.ts";

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

const versionDir = /^version-\d{8}T\d{6}Z-\d{3}$/;

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
  files["types.yaml"] = await resolveIncludes(fs, join(dir, "types.yaml"), files["types.yaml"]);
  const names = await fs.readdir(dir).catch(() => [] as string[]);
  for (const name of names) {
    if (!/^datasource\.[a-z0-9]+\.yaml$/.test(name)) continue;
    const text = await readOptional(fs, join(dir, name));
    if (text !== undefined) files[name] = await resolveIncludes(fs, join(dir, name), text);
  }
  files["datasource.yaml"] ??= defaultDatasourceYaml;
  files["datasource.yaml"] = await resolveIncludes(fs, join(dir, "datasource.yaml"), files["datasource.yaml"]);
  const diffs = names.filter((name) => versionDir.test(name)).sort();
  const versionFiles: string[] = [];
  for (const name of diffs) {
    const path = `${name}/diff.yaml`;
    const text = await readOptional(fs, join(dir, path));
    if (text === undefined) continue;
    files[path] = text;
    versionFiles.push(path);
  }
  return {
    files,
    settings: {
      ...settingsFrom(await readOptional(fs, join(dir, "settings.yaml"))),
      "paths.deterministic": dir,
      ...(versionFiles.length ? { "version.diffs": versionFiles.join("\n") } : {}),
    },
  };
};
