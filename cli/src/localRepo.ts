import { existsSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const cliSrc = dirname(fileURLToPath(import.meta.url));
const marker = "typescript/datasources/package.json";

export const cliDatasourcesRepo = resolve(cliSrc, "../..");

const isRepo = (dir: string): boolean => existsSync(join(dir, marker));

export const requireDatasourcesRepo = (start = process.cwd(), checkout = cliDatasourcesRepo): string => {
  const fromEnv = process.env.DATASOURCES_REPO;
  if (fromEnv !== undefined && fromEnv.length > 0 && isRepo(resolve(fromEnv))) return resolve(fromEnv);
  let dir = resolve(start);
  for (;;) {
    if (isRepo(dir)) return dir;
    const parent = dirname(dir);
    if (parent === dir) break;
    dir = parent;
  }
  if (isRepo(checkout)) return checkout;
  throw new Error("--local is for developing this datasources checkout");
};
