import { readFile } from "node:fs/promises";
import { join, resolve } from "node:path";

const rustCargoToml = join(resolve(import.meta.dirname, ".."), "rust/Cargo.toml");

export const cargoWorkspaceVersion = async (path = rustCargoToml) => {
  const match = (await readFile(path, "utf8")).match(/^version = "([^"]+)"/m);
  if (!match) {
    console.error("rust/Cargo.toml is missing workspace.package version");
    process.exit(1);
  }
  return match[1];
};
