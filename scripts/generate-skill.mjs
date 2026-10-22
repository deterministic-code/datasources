import { existsSync, mkdirSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { catalog, providers } from "../cli/src/catalog.ts";
import { transformDialects } from "./dialects.mjs";
import { MESSAGE } from "./hooks/block-generated.mjs";
import { parsePointer, slug, snippetFile } from "./sync-examples.mjs";
import { parseToc, uniquePages } from "./toc.mjs";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
export const SKILL_ROOTS = ["skills", ".cursor/skills/datasources"];
export const SKILL_REPO = "deterministic-code/datasources";
const BANNER = "<!-- Generated from docs/. Edit the markdown in docs/ and run npm run docs:skill. -->\n\n";
const SKIP_DOCS = new Set(["docs/examples.md", "docs/sections/where-statement.md"]);
const DOC_FENCE = /(^~~~~[\s\S]*?^~~~~)/m;
const CODE_FENCE = /(^```[\s\S]*?^```)/m;

const TOPICS = [
  {
    file: "downloading.md",
    title: "Downloading",
    sources: [{ path: "quickstart.md", sections: ["Installation from source"] }],
    extra: "packages",
  },
  {
    file: "setup.md",
    sources: [{ path: "cli.md", sections: ["Setup", "Add", "Remove", ".env"], intro: true }],
  },
  {
    file: "generation.md",
    title: "Repository generation",
    sources: [{ path: "cli.md", sections: ["Generate", "Migrations"] }, { path: "repository-generation.md" }],
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
  { file: "command.md", sources: [{ path: "sections/command.md" }, { path: "sections/native.md" }] },
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
  { path: "cli.md", hash: "migrations", file: "generation.md" },
  { path: "quickstart.md", hash: "installation-from-source", file: "downloading.md" },
];

// Skill text that is not in docs/. `rules` are always-on bullets on SKILL.md.
// `sections` are markdown appended to a topic file.
export const SKILL_ONLY = {
  rules: [
    "After you change `types.yaml` or `datasource.yaml`, run `version`, then `generate all` for the project's dialect (`csharp`, `typescript`, or `rust`). `version` writes the next `version-*/diff.yaml`. When it prints `unchanged`, stop.",
    "Do not create, edit, or delete a file under a directory named `generated`. Edit `types.yaml` or `datasource.yaml` and run `generate all`. In a consumer project, if `.cursor/hooks/block-generated.mjs` is missing, offer to install that hook and wait for a yes. generation.md lists the files to write. Skip that offer when the project root contains `scripts/generate-skill.mjs`.",
  ],
  sections: {
    "generation.md": `## After the spec changes

After you change \`types.yaml\` or \`datasource.yaml\`, snapshot the next version, then run \`generate all\`.

\`\`\`sh
npx @deterministic-code/datasources-cli version
npx @deterministic-code/datasources-cli generate all --dialect typescript
\`\`\`

\`version\` writes \`deterministic/version-<stamp>-<seq>/diff.yaml\` from the top-level YAML. \`--input\` selects the deterministic project (default \`./deterministic\`). An unchanged spec prints \`unchanged\` and writes nothing; stop there. \`generate all\` then reads those snapshots and writes the next migration. Pass \`--dialect csharp\`, \`typescript\`, or \`rust\`. Run it once per dialect when the project has more than one.

## Generated directory hook

Do not create, edit, or delete a file under a directory named \`generated\`. The hook replies with \`${MESSAGE}\`.

In a consumer project, if \`.cursor/hooks/block-generated.mjs\` is missing, offer to install the hook before editing and wait for a yes. Skip the offer when the project root contains \`scripts/generate-skill.mjs\`.

Copy \`hooks/block-generated.mjs\` from the same directory as this file to \`.cursor/hooks/block-generated.mjs\`. \`datasources-cli generate\` and \`npm run cli\` may still write there. Merge the entries below and keep every hook the project already has. Add a command only when that command is not already listed.

\`.cursor/hooks.json\`:

\`\`\`json
{
  "version": 1,
  "hooks": {
    "preToolUse": [
      { "command": "node .cursor/hooks/block-generated.mjs", "failClosed": true }
    ],
    "beforeShellExecution": [
      { "command": "node .cursor/hooks/block-generated.mjs", "failClosed": true }
    ]
  }
}
\`\`\`

\`.claude/settings.json\`:

\`\`\`json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Write|Edit|MultiEdit|NotebookEdit|Bash|PowerShell",
        "hooks": [
          {
            "type": "command",
            "command": "node \\"\${CLAUDE_PROJECT_DIR}/.cursor/hooks/block-generated.mjs\\""
          }
        ]
      }
    ]
  }
}
\`\`\`
`,
  },
};

const TOPICS_LINKS = [
  ["Install packages", "downloading.md"],
  ["setup / add / remove / .env", "setup.md"],
  ["Generate from types.yaml and datasource.yaml", "generation.md"],
  ["Mappers and the read pipeline", "mappings.md"],
  ["Type converters", "type-converters.md"],
  ["Command", "command.md"],
  ["Reader", "reader.md"],
  ["Repositories overview", "repositories.md"],
  ["ReadonlyRepository", "readonly-repository.md"],
  ["CRUDRepository", "crud-repository.md"],
  ["BulkRepository", "bulk-repository.md"],
  ["Repository factories", "repository-factories.md"],
  ["AddEnriched / AddLinked", "enrichment.md"],
  ["Statements", "statements.md"],
  ["Where", "where.md"],
  ["Expressions", "expressions.md"],
  ["SelectStatement", "select-statement.md"],
  ["InsertStatement", "insert-statement.md"],
  ["UpdateStatement", "update-statement.md"],
  ["DeleteStatement", "delete-statement.md"],
  ["Other statements", "other-statements.md"],
  ["Example index", "examples.md"],
];

export const skillRouter = (dir = "") => {
  const href = (file) => `${dir}${file}`;
  return `---
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
- When writing code, find the closest row in [examples.md](${href("examples.md")}), open that language's snippet, and use the same objects the sample uses (connection, repository, enrichment). Include every type the sample defines.
- \`AddEnriched\` is a read fill. A \`List\` on a CRUD repository is also written with the parent. \`AddLinked\` reads the related repository and writes the association through the link repository. Register a child repository's own fills on that child, then register the child on the parent. Read [enrichment.md](${href("enrichment.md")}) before either call.
${SKILL_ONLY.rules.map((rule) => `- ${rule}`).join("\n")}

## Open one file

| Task | File |
| --- | --- |
${TOPICS_LINKS.map(([task, file]) => `| ${task} | [${file}](${href(file)}) |`).join("\n")}
`;
};

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
      .replace(/<SkillPrompt\b[^>]*\/>\n*/g, "")
      .replace(/<SkillPrompt\b[^>]*>[\s\S]*?<\/SkillPrompt>\n*/g, "")
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
    "Install providers with `npx @deterministic-code/datasources-cli add <name>`. Do not invent package names.",
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
  const only = SKILL_ONLY.sections[topic.file];
  if (only) parts.push(only.trim());
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
  const files = { "SKILL.md": skillRouter() };
  for (const topic of TOPICS) files[topic.file] = renderTopic(topic, base, routes, pages);
  files["examples.md"] = renderExamples(base, pages);
  files["hooks/block-generated.mjs"] = readFileSync(join(base, "scripts/hooks/block-generated.mjs"), "utf8");
  return files;
};

const listFiles = (dir) =>
  existsSync(dir)
    ? readdirSync(dir, { withFileTypes: true }).flatMap((entry) => {
        const path = join(dir, entry.name);
        return entry.isDirectory() ? listFiles(path) : [path];
      })
    : [];

const pruneEmpty = (dir) => {
  if (!existsSync(dir)) return;
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    const path = join(dir, entry.name);
    if (!entry.isDirectory()) continue;
    pruneEmpty(path);
    if (!readdirSync(path).length) rmSync(path, { recursive: true });
  }
};

const writeTree = (dir, files) => {
  mkdirSync(dir, { recursive: true });
  for (const [name, body] of Object.entries(files)) {
    const path = join(dir, name);
    mkdirSync(dirname(path), { recursive: true });
    writeFileSync(path, body);
  }
  for (const path of listFiles(dir)) {
    const name = relative(dir, path).split("\\").join("/");
    if (!(name in files)) rmSync(path);
  }
  pruneEmpty(dir);
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

export const applySkill = ({ check = false, base = root } = {}) => {
  const files = renderSkill(base);
  const topics = Object.fromEntries(Object.entries(files).filter(([name]) => name !== "SKILL.md"));
  const rootSkill = skillRouter("skills/");
  const rootPath = join(base, "SKILL.md");
  const copies = SKILL_ROOTS.filter((rel) => rel !== "skills").map((rel) => join(base, rel));
  const topicsDir = join(base, "skills");
  if (check) {
    const errors = [
      ...checkTree(topicsDir, topics),
      ...copies.flatMap((dir) => checkTree(dir, files)),
      ...(existsSync(rootPath) && readFileSync(rootPath, "utf8") === rootSkill
        ? []
        : [`${rootPath} is stale. Run npm run docs:skill.`]),
    ];
    if (errors.length) {
      for (const error of errors) console.error(error);
      process.exit(1);
    }
    return files;
  }
  writeFileSync(rootPath, rootSkill);
  writeTree(topicsDir, topics);
  for (const dir of copies) writeTree(dir, files);
  console.log(`Wrote SKILL.md and ${["skills", ...SKILL_ROOTS.filter((rel) => rel !== "skills")].join(" and ")}`);
  return files;
};

const main = process.argv[1] && fileURLToPath(import.meta.url) === pathToFileURL(process.argv[1]).pathname;
if (main || process.argv[1]?.endsWith("generate-skill.mjs"))
  applySkill({ check: process.argv.includes("--check") });
