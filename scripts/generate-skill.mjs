import { existsSync, mkdirSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { crc32, deflateRawSync } from "node:zlib";
import { catalog, providers } from "../cli/src/catalog.ts";
import { transformDialects } from "./dialects.mjs";
import { parsePointer, slug, snippetFile } from "./sync-examples.mjs";
import { parseToc, uniquePages } from "./toc.mjs";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
export const SKILL_ROOTS = ["skills/datasources", ".cursor/skills/datasources", ".claude/skills/datasources"];
export const SKILL_ZIPS = ["docs/public/datasources-skill.zip"];
const BANNER = "<!-- Generated from docs/. Edit the markdown in docs/ and run npm run docs:skill. -->\n\n";
const SKIP_DOCS = new Set(["docs/examples.md", "docs/sections/where-statement.md"]);
const DOC_FENCE = /(^~~~~[\s\S]*?^~~~~)/m;
const CODE_FENCE = /(^```[\s\S]*?^```)/m;

const TOPICS = [
  {
    file: "downloading.md",
    title: "Downloading",
    sources: [{ path: "quickstart.md", sections: ["Installation"] }],
    extra: "packages",
  },
  {
    file: "setup.md",
    sources: [{ path: "cli.md", sections: ["Setup", "Add", "Remove", ".env"], intro: true }],
  },
  {
    file: "generation.md",
    title: "Repository generation",
    sources: [{ path: "cli.md", sections: ["Generate"] }, { path: "repository-generation.md" }],
  },
  {
    file: "mappings.md",
    sources: [
      { path: "tutorials/mapping-converting-pipelines.md" },
      { path: "sections/mappers.md" },
      { path: "sections/identity-mapper.md" },
      { path: "sections/dictionary-mapper.md" },
      { path: "sections/pluralizer-mapper.md" },
      { path: "sections/change-casing-mapper.md" },
      { path: "sections/transform-mapper.md" },
      { path: "sections/flat-to-nested-mapper.md" },
    ],
  },
  {
    file: "type-converters.md",
    sources: [{ path: "sections/type-converters.md" }, { path: "sections/metadata-row-type-converter.md" }],
  },
  { file: "command.md", sources: [{ path: "sections/command.md" }] },
  { file: "reader.md", sources: [{ path: "sections/reader.md" }] },
  { file: "repositories.md", sources: [{ path: "sections/repository.md" }] },
  { file: "readonly-repository.md", sources: [{ path: "sections/readonly-repository.md" }] },
  { file: "crud-repository.md", sources: [{ path: "sections/crud-repository.md" }] },
  { file: "bulk-repository.md", sources: [{ path: "sections/bulk-repository.md" }] },
  { file: "repository-factories.md", sources: [{ path: "sections/repository-factories.md" }] },
  {
    file: "enrichment.md",
    sources: [{ path: "sections/i-enriched.md" }, { path: "sections/enrichment.md" }],
  },
  { file: "statements.md", sources: [{ path: "sections/statements.md" }] },
  { file: "where.md", sources: [{ path: "sections/where-clause.md" }] },
  {
    file: "expressions.md",
    sources: [
      { path: "sections/expressions.md" },
      { path: "sections/expression-evaluator.md" },
      { path: "sections/statement-evaluator.md" },
    ],
  },
  { file: "select-statement.md", sources: [{ path: "sections/select-statement.md" }] },
  { file: "insert-statement.md", sources: [{ path: "sections/insert-statement.md" }] },
  { file: "update-statement.md", sources: [{ path: "sections/update-statement.md" }] },
  { file: "delete-statement.md", sources: [{ path: "sections/delete-statement.md" }] },
  {
    file: "other-statements.md",
    title: "Other statements",
    sources: [
      { path: "sections/create-table-statement.md" },
      { path: "sections/create-index-statement.md" },
      { path: "sections/execute-stored-procedure-statement.md" },
      { path: "sections/return-statement.md" },
    ],
  },
];

const HASH_ROUTES = [
  { path: "cli.md", hash: "generate", file: "generation.md" },
  { path: "quickstart.md", hash: "installation", file: "downloading.md" },
];

const SKILL_MD = `---
name: datasources
description: >-
  Uses Datasources across C#, Rust, and TypeScript: install packages, run setup
  and add providers, generate repositories from types.yaml and datasource.yaml,
  write mappers and type converters, run commands and readers, build
  repositories, call AddEnriched and AddLinked, and write Where and other
  statements from the documented examples. Use when the user mentions
  datasources, repository generation, types.yaml, datasource.yaml, mapper,
  type converter, command, reader, AddEnriched, AddLinked, Where, or
  SelectStatement.
---

${BANNER}# Datasources

C# is the reference. Rust and TypeScript match its call shape (\`Read\` / \`read\` / \`read\`).

Read one file below. Do not load the rest.

## Rules

- A select names PascalCase entity types and properties (\`Contact.FirstName\`), not table or column names.
- Read is convert, then map. Do not fold mapping into the converter.
- When writing code, find the closest row in [examples.md](examples.md), open that language's snippet, and use the same objects the sample uses (connection, repository, enrichment). Include every type the sample defines.
- \`AddEnriched\` is a read fill. A \`List\` on a CRUD repository is also written with the parent. \`AddLinked\` reads the related repository and writes the association through the link repository. Register a child repository's own fills on that child, then register the child on the parent. Read [enrichment.md](enrichment.md) before either call.

## Open one file

| Task | File |
| --- | --- |
| Install packages | [downloading.md](downloading.md) |
| setup / add / remove / .env | [setup.md](setup.md) |
| Generate from types.yaml and datasource.yaml | [generation.md](generation.md) |
| Mappers and the read pipeline | [mappings.md](mappings.md) |
| Type converters | [type-converters.md](type-converters.md) |
| Command | [command.md](command.md) |
| Reader | [reader.md](reader.md) |
| Repositories overview | [repositories.md](repositories.md) |
| ReadonlyRepository | [readonly-repository.md](readonly-repository.md) |
| CRUDRepository | [crud-repository.md](crud-repository.md) |
| BulkRepository | [bulk-repository.md](bulk-repository.md) |
| Repository factories | [repository-factories.md](repository-factories.md) |
| AddEnriched / AddLinked | [enrichment.md](enrichment.md) |
| Statements | [statements.md](statements.md) |
| Where | [where.md](where.md) |
| Expressions | [expressions.md](expressions.md) |
| SelectStatement | [select-statement.md](select-statement.md) |
| InsertStatement | [insert-statement.md](insert-statement.md) |
| UpdateStatement | [update-statement.md](update-statement.md) |
| DeleteStatement | [delete-statement.md](delete-statement.md) |
| Other statements | [other-statements.md](other-statements.md) |
| Example index | [examples.md](examples.md) |
`;

const outsideFences = (markdown, fn) =>
  markdown
    .split(CODE_FENCE)
    .map((chunk) => (chunk.startsWith("```") ? chunk : fn(chunk)))
    .join("");

const demoteHeadings = (markdown) =>
  outsideFences(markdown, (text) => text.replaceAll(/^(#{1,5}) /gm, (_, hashes) => `${hashes}# `));

const stripFrontmatter = (text) => text.replace(/^---\r?\n[\s\S]*?\r?\n---\r?\n*/, "");

const dropHtml = (markdown) =>
  outsideFences(markdown, (text) =>
    text
      .replace(/<DialectSelector\s*\/>\n*/g, "")
      .replace(/<\/?(?:div|a|img|span)[^>]*>\n*/g, ""),
  );

const escapeRe = (value) => value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");

const extractSource = (markdown, { sections, intro = false } = {}) => {
  if (!sections) return markdown.trim();
  const parts = [];
  if (intro) {
    const first = markdown.search(/^## /m);
    if (first > 0) parts.push(markdown.slice(0, first).trim());
  }
  for (const title of sections) {
    const match = markdown.match(new RegExp(`^## ${escapeRe(title)}\\s*$`, "m"));
    if (!match) throw new Error(`Missing ## ${title}`);
    const start = match.index;
    const rest = markdown.slice(start + match[0].length);
    const next = rest.search(/^## /m);
    parts.push((next < 0 ? markdown.slice(start) : markdown.slice(start, start + match[0].length + next)).trim());
  }
  return parts.join("\n\n");
};

const readDoc = (base, path) => stripFrontmatter(readFileSync(join(base, "docs", path), "utf8"));

const normalizeHref = (href) => {
  let file = href.split("#")[0];
  if (file.startsWith("docs/")) file = file.slice(5);
  while (file.startsWith("./")) file = file.slice(2);
  while (file.startsWith("../")) file = file.slice(3);
  return file;
};

const pathMatches = (route, href) => {
  const file = normalizeHref(href);
  return route === file || route.endsWith(`/${file}`) || route.split("/").pop() === file;
};

const routesFor = (topics) => {
  const routes = [];
  for (const topic of topics) {
    for (const source of topic.sources) routes.push({ path: source.path, file: topic.file });
  }
  return [...HASH_ROUTES, ...routes];
};

const skillFor = (href, hash, routes, pages) => {
  const file = normalizeHref(href);
  const matches = routes.filter((route) => pathMatches(route.path, file));
  const hashed = hash ? matches.find((route) => route.hash === hash.slice(1)) : null;
  if (hashed) return hashed.file;
  const open = matches.find((route) => !route.hash);
  if (open) return open.file;
  if (matches[0]) return matches[0].file;
  const page = pages.find((entry) => pathMatches(entry.path, file));
  return `docs/${page?.path ?? file}`;
};

const rewriteLinks = (markdown, current, routes, pages) =>
  outsideFences(markdown, (text) =>
    text.replace(/\]\(([^)]+)\)/g, (full, href) => {
      if (href.startsWith("http") || href.startsWith("#")) return full;
      const [path, hash = ""] = href.split("#");
      if (!path.endsWith(".md")) return full;
      const target = skillFor(path, hash ? `#${hash}` : "", routes, pages);
      const suffix = hash ? `#${hash}` : "";
      if (target === current) return `](${suffix || "#"})`;
      return `](${target}${suffix})`;
    }),
  );

const packagesTable = () =>
  [
    "## Provider packages",
    "",
    "Install providers with `npx @datasources/cli add <name>`. Do not invent package names.",
    "",
    "| Provider | npm | cargo | nuget |",
    "| --- | --- | --- | --- |",
    ...providers.map((name) => `| ${name} | ${catalog[name].npm} | ${catalog[name].cargo} | ${catalog[name].nuget} |`),
    "",
  ].join("\n");

const renderTopic = (topic, base, routes, pages) => {
  const parts = topic.sources.map((source) =>
    transformDialects(extractSource(readDoc(base, source.path), source), "readme"),
  );
  if (topic.extra === "packages") parts.push(packagesTable());
  const nest = (part, index) => (index === 0 && !topic.title ? part : demoteHeadings(part));
  const body = topic.title
    ? `# ${topic.title}\n\n${parts.map((part) => (part.startsWith("##") ? part : demoteHeadings(part))).join("\n\n")}`
    : parts.map(nest).join("\n\n");
  return `${BANNER}${dropHtml(rewriteLinks(body.trim(), topic.file, routes, pages))}\n`;
};

const samplesIn = (markdown) => {
  const samples = [];
  for (const part of markdown.split(DOC_FENCE)) {
    if (!part.startsWith("~~~~")) {
      const lines = part.split("\n");
      let heading = null;
      let skip = 0;
      let inCode = false;
      for (const line of lines) {
        if (line.startsWith("```")) {
          if (!inCode) {
            const match = line.match(/^```([a-zA-Z0-9_+-]+)\s+(\S+#\S+)\s*$/);
            if (match) samples.push({ heading, lang: match[1], spec: match[2] });
          }
          inCode = !inCode;
        } else if (!inCode && (line.startsWith("::: prose") || line.startsWith("::: reference"))) skip++;
        else if (!inCode && line.trim() === ":::" && skip) skip--;
        else if (!inCode && !skip) {
          const title = line.match(/^###\s+(.+?)\s*$/);
          if (title) heading = title[1];
        }
      }
    }
  }
  return samples;
};

const walkMd = (dir) => {
  const files = [];
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    const path = join(dir, entry.name);
    if (entry.isDirectory()) files.push(...walkMd(path));
    else if (entry.name.endsWith(".md")) files.push(path);
  }
  return files;
};

const renderExamples = (base, pages) => {
  const byDoc = new Map();
  for (const doc of walkMd(join(base, "docs"))) {
    const rel = relative(base, doc).split("\\").join("/");
    if (SKIP_DOCS.has(rel)) continue;
    const seen = new Map();
    for (const sample of samplesIn(readFileSync(doc, "utf8"))) {
      if (!sample.heading) continue;
      const items = seen.get(sample.heading) ?? [];
      const pointer = parsePointer(sample.spec);
      const file = snippetFile({
        heading: sample.heading,
        sourcePath: pointer.path,
        name: pointer.name,
        lang: sample.lang,
      });
      if (!items.includes(file)) items.push(file);
      seen.set(sample.heading, items);
    }
    if (seen.size) byDoc.set(rel, seen);
  }
  const docOf = (path) => (path.startsWith("docs/") ? path : `docs/${path}`);
  const groups = [];
  const used = new Set();
  for (const page of uniquePages(pages)) {
    const doc = docOf(page.path);
    const items = byDoc.get(doc);
    if (!items) continue;
    used.add(doc);
    groups.push({ title: page.title, items });
  }
  for (const [doc, items] of byDoc) {
    if (used.has(doc)) continue;
    groups.push({ title: doc.replace(/^docs\//, "").replace(/\.md$/, ""), items });
  }
  const body = groups
    .map(
      (group) =>
        `## ${group.title}\n\n${[...group.items.entries()]
          .map(([title, files]) => `- ${title}\n${files.map((file) => `  - \`${file}\``).join("\n")}`)
          .join("\n")}`,
    )
    .join("\n\n");
  return `${BANNER}# Examples\n\nFind the closest title. Open that language's snippet. Topic files already include the expanded samples.\n\n${body}\n`;
};

export const renderSkill = (base = root) => {
  const { pages } = parseToc(readFileSync(join(base, "docs/toc.yaml"), "utf8"));
  const routes = routesFor(TOPICS);
  const files = { "SKILL.md": SKILL_MD };
  for (const topic of TOPICS) files[topic.file] = renderTopic(topic, base, routes, pages);
  files["examples.md"] = renderExamples(base, pages);
  return files;
};

const listFiles = (dir) =>
  existsSync(dir)
    ? readdirSync(dir, { withFileTypes: true }).flatMap((entry) => {
        const path = join(dir, entry.name);
        return entry.isDirectory() ? listFiles(path) : [path];
      })
    : [];

const u16 = (n) => {
  const buf = Buffer.alloc(2);
  buf.writeUInt16LE(n);
  return buf;
};

const u32 = (n) => {
  const buf = Buffer.alloc(4);
  buf.writeUInt32LE(n);
  return buf;
};

export const zipSkill = (files) => {
  const entries = Object.keys(files)
    .sort()
    .map((name) => ({ name: `datasources/${name}`, data: Buffer.from(files[name]) }));
  const locals = [];
  const centrals = [];
  let offset = 0;
  for (const { name, data } of entries) {
    const compressed = deflateRawSync(data);
    const crc = crc32(data);
    const nameBuf = Buffer.from(name);
    const local = Buffer.concat([
      Buffer.from("PK\x03\x04"),
      u16(20),
      u16(0),
      u16(8),
      u16(0),
      u16(0),
      u32(crc),
      u32(compressed.length),
      u32(data.length),
      u16(nameBuf.length),
      u16(0),
      nameBuf,
      compressed,
    ]);
    locals.push(local);
    centrals.push(
      Buffer.concat([
        Buffer.from("PK\x01\x02"),
        u16(20),
        u16(20),
        u16(0),
        u16(8),
        u16(0),
        u16(0),
        u32(crc),
        u32(compressed.length),
        u32(data.length),
        u16(nameBuf.length),
        u16(0),
        u16(0),
        u16(0),
        u16(0),
        u32(0),
        u32(offset),
        nameBuf,
      ]),
    );
    offset += local.length;
  }
  const center = Buffer.concat(centrals);
  return Buffer.concat([
    ...locals,
    center,
    Buffer.from("PK\x05\x06"),
    u16(0),
    u16(0),
    u16(entries.length),
    u16(entries.length),
    u32(center.length),
    u32(offset),
    u16(0),
  ]);
};

const writeTree = (dir, files) => {
  mkdirSync(dir, { recursive: true });
  for (const [name, body] of Object.entries(files)) writeFileSync(join(dir, name), body);
  for (const path of listFiles(dir)) {
    const name = relative(dir, path).split("\\").join("/");
    if (!(name in files)) rmSync(path);
  }
};

const checkTree = (dir, files) => {
  const errors = [];
  if (!existsSync(dir)) return [`${dir} is missing. Run npm run docs:skill.`];
  for (const [name, body] of Object.entries(files)) {
    const path = join(dir, name);
    if (!existsSync(path)) errors.push(`${path} is missing. Run npm run docs:skill.`);
    else if (readFileSync(path, "utf8") !== body) errors.push(`${path} is stale. Run npm run docs:skill.`);
  }
  for (const path of listFiles(dir)) {
    const name = relative(dir, path).split("\\").join("/");
    if (!(name in files)) errors.push(`${path} is extra. Run npm run docs:skill.`);
  }
  return errors;
};

const checkZip = (path, zip) => {
  if (!existsSync(path)) return [`${path} is missing. Run npm run docs:skill.`];
  return Buffer.compare(readFileSync(path), zip) === 0 ? [] : [`${path} is stale. Run npm run docs:skill.`];
};

export const applySkill = ({ check = false, base = root } = {}) => {
  const files = renderSkill(base);
  const dirs = SKILL_ROOTS.map((rel) => join(base, rel));
  const zip = zipSkill(files);
  const zips = SKILL_ZIPS.map((rel) => join(base, rel));
  if (check) {
    const errors = [...dirs.flatMap((dir) => checkTree(dir, files)), ...zips.flatMap((path) => checkZip(path, zip))];
    if (errors.length) {
      for (const error of errors) console.error(error);
      process.exit(1);
    }
    return files;
  }
  for (const dir of dirs) writeTree(dir, files);
  for (const path of zips) {
    mkdirSync(dirname(path), { recursive: true });
    writeFileSync(path, zip);
  }
  console.log(`Wrote ${[...dirs, ...zips].map((path) => relative(base, path)).join(" and ")}`);
  return files;
};

const main = process.argv[1] && fileURLToPath(import.meta.url) === pathToFileURL(process.argv[1]).pathname;
if (main || process.argv[1]?.endsWith("generate-skill.mjs"))
  applySkill({ check: process.argv.includes("--check") });
