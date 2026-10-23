import { execFileSync } from "node:child_process";
import { existsSync, lstatSync, readdirSync, rmSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const SKIP = new Set([".git", "node_modules", "target", "bin", "obj", "deterministic-generators-common"]);
const DROP_DIRS = [
  "scripts/artifacts",
  "csharp/artifacts",
  "docs/evals/workspace",
  "docs/.vitepress/cache",
  "docs/.vitepress/dist",
  ".deterministic-cache",
];
const DROP_DIR_NAMES = new Set(["coverage", "TestResults"]);
const DROP_FILES = new Set(["lcov.info", "coverage.cobertura.xml", "coverage.info"]);
const DB = /^(app\.sqlite|app\.db)(-wal|-shm|-journal)?$/;
const BUILD = /\.(tsbuildinfo|nupkg)$/;

const posix = (root, path) => relative(root, path).split("\\").join("/");

export const tracked = (cwd) =>
  new Set(execFileSync("git", ["ls-files", "-z"], { cwd, encoding: "utf8" }).split("\0").filter(Boolean));

export const clean = (root, kept) => {
  const drop = (path) => {
    if (!existsSync(path) || kept.has(posix(root, path))) return;
    rmSync(path, { recursive: true, force: true });
  };
  for (const dir of DROP_DIRS) drop(join(root, dir));
  walk(root, root, drop);
};

const walk = (root, dir, drop) => {
  let names;
  try {
    names = readdirSync(dir);
  } catch {
    return;
  }
  for (const name of names) {
    if (SKIP.has(name)) continue;
    const path = join(dir, name);
    let stat;
    try {
      stat = lstatSync(path);
    } catch {
      continue;
    }
    if (stat.isDirectory()) {
      if (DROP_DIR_NAMES.has(name)) {
        drop(path);
        continue;
      }
      if (name === "var") {
        try {
          if (readdirSync(path).includes("folders")) {
            drop(path);
            continue;
          }
        } catch {
          continue;
        }
      }
      walk(root, path, drop);
      continue;
    }
    if (DROP_FILES.has(name) || BUILD.test(name) || DB.test(name)) drop(path);
  }
};

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
if (import.meta.url === pathToFileURL(process.argv[1]).href) clean(root, tracked(root));
