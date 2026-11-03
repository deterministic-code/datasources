import { existsSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const cliSrc = dirname(fileURLToPath(import.meta.url));
const marker = "typescript/datasources/package.json";

export const cliDatasourcesRepo = resolve(cliSrc, "../..");

export const requireDatasourcesRepo = (start = process.cwd()): string => {
  let dir = resolve(start);
  for (;;) {
    if (existsSync(join(dir, marker))) return dir;
    const parent = dirname(dir);
    if (parent === dir) throw new Error("--local is for developing this datasources checkout");
    dir = parent;
  }
};
