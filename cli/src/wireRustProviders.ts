import { dirname, join } from "node:path";
import { catalog, type Provider } from "./catalog.ts";
import type { Fs } from "./io.ts";

const optional = async (fs: Fs, path: string): Promise<string | undefined> => {
  try {
    return await fs.readFile(path);
  } catch {
    return undefined;
  }
};

const firstFile = async (fs: Fs, cwd: string, names: string[]): Promise<{ path: string; text: string } | undefined> => {
  for (const name of names) {
    const path = join(cwd, name);
    const text = await optional(fs, path);
    if (text !== undefined) return { path, text };
  }
  return undefined;
};

export const rustBindContents = (names: Provider[]): string =>
  names.map((name) => `use ${catalog[name].cargo.replaceAll("-", "_")} as _;\n`).join("");

export const attachedProviders = (entry: string): string | undefined =>
  entry.includes("datasources_providers") ? undefined : `mod datasources_providers;\n${entry}`;

export const wireRustProviders = async (
  cwd: string,
  names: Provider[],
  dryRun: boolean,
  deps: { fs: Fs; log: (line: string) => void },
): Promise<void> => {
  const entry = await firstFile(deps.fs, cwd, ["src/lib.rs", "src/main.rs", "lib.rs", "main.rs"]);
  const bind = join(entry ? dirname(entry.path) : cwd, "datasources_providers.rs");
  const text = rustBindContents(names);
  if (dryRun) {
    deps.log(bind);
    return;
  }
  await deps.fs.writeFile(bind, text);
  deps.log(bind);
  if (!entry) return;
  const next = attachedProviders(entry.text);
  if (next === undefined) return;
  await deps.fs.writeFile(entry.path, next);
  deps.log(entry.path);
};
