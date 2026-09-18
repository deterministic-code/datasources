import { readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { transformDialects } from "./dialects.mjs";
import { parseToc, uniquePages } from "./toc.mjs";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const docsDir = join(root, "docs");
const readmePath = join(root, "README.md");
const banner =
  "<!-- Generated from docs/. Edit the markdown in docs/ and run npm run docs:readme. -->\n\n";

const transformOutsideFences = (markdown, fn) =>
  markdown
    .split(/(^```[\s\S]*?^```)/m)
    .map((chunk) => (chunk.startsWith("```") ? chunk : fn(chunk)))
    .join("");

const demoteHeadings = (markdown) =>
  transformOutsideFences(markdown, (text) => text.replaceAll(/^(#{1,5}) /gm, (_, hashes) => `${hashes}# `));

const slug = (title) =>
  title
    .toLowerCase()
    .replace(/[^\w\s-]/g, "")
    .trim()
    .replace(/\s+/g, "-");

const rewriteDocLinks = (markdown, pages) =>
  markdown.replace(/\]\(([^)]+\.md)(#[^)]*)?\)/g, (full, href, hash) => {
      const file = href.replace(/^\.\//, "").replace(/^\.\.\//, "");
      const page = pages.find(
        (entry) => entry.path === file || entry.path.endsWith(`/${file}`) || entry.path.split("/").pop() === file,
      );
      return page ? `](#${hash ? hash.slice(1) : slug(page.title)})` : full;
    });

const generate = () => {
  const { pages } = parseToc(readFileSync(join(docsDir, "toc.yaml"), "utf8"));
  const body = uniquePages(pages)
    .map(({ path }, index) => {
      const source = readFileSync(join(docsDir, path), "utf8").trim();
      const shifted = index === 0 ? source : demoteHeadings(source);
      return rewriteDocLinks(transformDialects(shifted, "readme"), pages);
    })
    .join("\n\n");
  return `${banner}${body}\n`;
};

const next = generate();
if (process.argv.includes("--check")) {
  const current = readFileSync(readmePath, "utf8");
  if (current !== next) {
    console.error("README.md is stale. Run npm run docs:readme.");
    process.exit(1);
  }
  process.exit(0);
}

writeFileSync(readmePath, next);
console.log("Wrote README.md");
