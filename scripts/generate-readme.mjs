import { readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { transformDialects } from "./dialects.mjs";
import { nestSidebar, parseToc, uniquePages } from "./toc.mjs";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const docsDir = join(root, "docs");
const readmePath = join(root, "README.md");
const banner =
  "<!-- Generated from docs/. Edit the markdown in docs/ and run npm run docs:readme. -->\n\n";

const README_PATHS = new Set([
  "introduction.md",
  "quickstart.md",
  "skill.md",
  "features.md",
  "repository-generation.md",
  "drivers.md",
]);

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

const docHref = (page, hash) => `docs/${page.path}${hash ?? ""}`;

const rewriteDocLinks = (markdown, pages, inlined) =>
  markdown.replace(/\]\(([^)]+\.md)(#[^)]*)?\)/g, (full, href, hash) => {
    const file = href.replace(/^\.\//, "").replace(/^\.\.\//, "");
    const page = pages.find(
      (entry) => entry.path === file || entry.path.endsWith(`/${file}`) || entry.path.split("/").pop() === file,
    );
    if (!page) return full;
    return inlined.has(page.path)
      ? `](#${hash ? hash.slice(1) : slug(page.title)})`
      : `](${docHref(page, hash)})`;
  });

const renderNav = (items, depth = 0) =>
  items
    .map((item) => {
      const line = `${"  ".repeat(depth)}- [${item.text}](${item.link})`;
      return item.items?.length ? `${line}\n${renderNav(item.items, depth + 1)}` : line;
    })
    .join("\n");

const moreDocs = (pages) => {
  const rest = pages.filter((page) => !README_PATHS.has(page.path));
  const groups = [...new Set(rest.map((page) => page.group).filter(Boolean))];
  const ordered = [
    ...groups.filter((group) => group === "Articles"),
    ...groups.filter((group) => group === "Code Reference"),
    ...groups.filter((group) => group !== "Articles" && group !== "Code Reference"),
  ];
  const ungrouped = uniquePages(rest.filter((page) => !page.group));
  const toItem = (page) => ({
    text: page.title,
    link: docHref(page, page.hash ? `#${page.hash}` : ""),
  });
  const parts = ["## Documentation"];
  if (ungrouped.length) parts.push(renderNav(ungrouped.map(toItem)));
  for (const group of ordered) {
    parts.push(`### ${group}\n\n${renderNav(nestSidebar(rest.filter((page) => page.group === group), toItem))}`);
  }
  return parts.join("\n\n");
};

const generate = () => {
  const { pages } = parseToc(readFileSync(join(docsDir, "toc.yaml"), "utf8"));
  const inlined = uniquePages(pages).filter((page) => README_PATHS.has(page.path));
  const body = inlined
    .map(({ path }, index) => {
      const source = readFileSync(join(docsDir, path), "utf8").trim();
      const transformed = transformDialects(source, "readme");
      const shifted = index === 0 ? transformed : demoteHeadings(transformed);
      return rewriteDocLinks(shifted, pages, README_PATHS);
    })
    .join("\n\n");
  return `${banner}${body}\n\n${moreDocs(pages)}\n`;
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
