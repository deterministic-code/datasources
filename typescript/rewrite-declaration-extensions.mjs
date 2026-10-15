import { readdir, readFile, writeFile } from "node:fs/promises";
import { join } from "node:path";

const dist = process.argv[2];
if (!dist) {
  console.error("usage: node rewrite-declaration-extensions.mjs <dist>");
  process.exit(1);
}

const rewrite = (source) =>
  source.replace(
    /(from\s+["'])(\.\.?\/[^"']+)\.ts(["'])/g,
    "$1$2.js$3",
  ).replace(
    /(import\(\s*["'])(\.\.?\/[^"']+)\.ts(["'])/g,
    "$1$2.js$3",
  );

const walk = async (dir) => {
  for (const entry of await readdir(dir, { withFileTypes: true })) {
    const path = join(dir, entry.name);
    if (entry.isDirectory()) {
      await walk(path);
      continue;
    }
    if (!entry.name.endsWith(".d.ts")) continue;
    const source = await readFile(path, "utf8");
    const next = rewrite(source);
    if (next !== source) await writeFile(path, next);
    if (/(?:from|import\(\s*)["']\.\.?\/[^"']+\.ts["']/.test(next)) {
      console.error(`${path} still imports a .ts path`);
      process.exit(1);
    }
  }
};

await walk(dist);
