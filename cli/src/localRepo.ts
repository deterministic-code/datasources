import { existsSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const cliSrc = dirname(fileURLToPath(import.meta.url));

export const cliDatasourcesRepo = resolve(cliSrc, "../..");

export const requireDatasourcesRepo = (root = cliDatasourcesRepo): string => {
  if (!existsSync(join(root, "typescript/datasources/package.json")))
    throw new Error("--local is for developing this datasources checkout");
  return root;
};
