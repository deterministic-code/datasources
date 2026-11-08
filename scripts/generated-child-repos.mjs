import { readdirSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

export const MESSAGE = "child fields must be generated *Repository types";

const CHILD = [
  /pub \{\{field\}\}: Arc<CRUDRepository/,
  /pub [a-zA-Z_][a-zA-Z0-9_]*: Arc<CRUDRepository</,
  /readonly [a-zA-Z_][a-zA-Z0-9_]*: (?:ICRUDRepository|CRUDRepository)</,
  /public (?:ICRUDRepository|CRUDRepository)<[^>]+> [a-zA-Z_][a-zA-Z0-9_]* \{ get; \}/,
];

export const childRepoViolations = (text) =>
  text.split("\n").flatMap((line, index) => {
    if (/pub\(crate\) inner:/.test(line) || /type Target = CRUDRepository/.test(line)) return [];
    return CHILD.filter((pattern) => pattern.test(line)).map((pattern) => ({ line: index + 1, pattern: String(pattern), text: line.trim() }));
  });

const walk = (root, suffix, found = []) => {
  for (const entry of readdirSync(root, { withFileTypes: true })) {
    const path = join(root, entry.name);
    if (entry.isDirectory()) {
      if (entry.name === "node_modules" || entry.name === "target" || entry.name === "bin" || entry.name === "obj") continue;
      walk(path, suffix, found);
      continue;
    }
    if (suffix.some((end) => entry.name.endsWith(end) || /Repository/i.test(entry.name))) found.push(path);
  }
  return found;
};

export const scanPaths = (files) =>
  files.flatMap((file) => childRepoViolations(readFileSync(file, "utf8")).map((hit) => ({ file, ...hit })));

export const repoFiles = (root) => {
  const templates = [
    join(root, "cli/src/generate/templates/csharp/repository.cs.tmpl"),
    join(root, "cli/src/generate/templates/typescript/repository.ts.tmpl"),
    join(root, "cli/src/generate/templates/rust/repository.rs.tmpl"),
  ];
  const generated = walk(join(root, "samples"), [".cs", ".ts", ".rs"]).filter((path) =>
    /[/\\]generated[/\\].*Repositor/i.test(path),
  );
  return [...templates, ...generated];
};

const here = dirname(fileURLToPath(import.meta.url));
if (process.argv[1] && fileURLToPath(import.meta.url) === process.argv[1]) {
  const hits = scanPaths(repoFiles(join(here, "..")));
  if (hits.length) {
    for (const hit of hits) console.error(`${hit.file}:${hit.line}: ${hit.text}`);
    console.error(MESSAGE);
    process.exit(1);
  }
}
