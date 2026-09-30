import { mkdirSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { exampleCode } from "./example.mjs";
import { parseToc, uniquePages } from "./toc.mjs";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const DOC_FENCE = /(^~~~~[\s\S]*?^~~~~)/m;
const RANGE = /^(.+)#L(\d+)-L(\d+)$/;
const NAMED = /^(.+)#([A-Za-z0-9_.-]+)$/;
const LANGS = {
  cs: "csharp",
  csharp: "csharp",
  rs: "rust",
  rust: "rust",
  ts: "typescript",
  typescript: "typescript",
};

const norm = (text) => text.replace(/\r\n/g, "\n");

export const slug = (title) =>
  title
    .toLowerCase()
    .replace(/[^\w\s-]/g, "")
    .trim()
    .replace(/\s+/g, "-");

export const normalizeLang = (lang) => LANGS[lang.toLowerCase()] ?? lang.toLowerCase();

export const parsePointer = (spec) => {
  const range = spec.match(RANGE);
  if (range) return { path: range[1], from: Number(range[2]), to: Number(range[3]), name: null };
  const named = spec.match(NAMED);
  if (!named) throw new Error(`Invalid example pointer: ${spec}`);
  return { path: named[1], name: named[2], from: null, to: null };
};

export const fileStem = (sourcePath) => {
  const parts = sourcePath.split("/");
  const stem = parts.pop().replace(/\.[^.]+$/, "");
  if (!["mod", "lib", "index"].includes(stem)) return stem;
  const parent = [...parts].reverse().find((part) => part !== "src" && part !== "tests");
  return `${parent}-${stem}`;
};

export const snippetFile = ({ heading, sourcePath, name, lang }) =>
  `docs/examples/${slug(heading)}/${fileStem(sourcePath)}.${name}.${normalizeLang(lang)}`;

export const LINES = "docs/examples/lines";
export const EXAMPLES_PAGE = "docs/examples.md";

const EXAMPLES_INTRO =
  "Samples from the code reference. Open a link for the explanation and the C#, Rust, and TypeScript code.";

export const renderExamplesPage = ({ pages, headings }) => {
  const byDoc = new Map();
  for (const heading of headings) {
    const items = byDoc.get(heading.doc) ?? [];
    if (!items.some((item) => item.title === heading.title)) items.push(heading);
    byDoc.set(heading.doc, items);
  }
  const docOf = (path) => (path.startsWith("docs/") ? path : `docs/${path}`);
  const groups = [];
  const seen = new Set();
  for (const page of uniquePages(pages)) {
    const doc = docOf(page.path);
    const items = byDoc.get(doc);
    if (!items?.length) continue;
    seen.add(doc);
    groups.push({ title: page.title, doc, items });
  }
  for (const [doc, items] of byDoc) {
    if (seen.has(doc)) continue;
    groups.push({ title: doc.replace(/^docs\//, "").replace(/\.md$/, ""), doc, items });
  }
  const sections = groups
    .map(
      (group) =>
        `## ${group.title}\n\n${group.items
          .map((item) => `- [${item.title}](./${group.doc.replace(/^docs\//, "")}#${slug(item.title)})`)
          .join("\n")}`,
    )
    .join("\n\n");
  return `# Examples\n\n${EXAMPLES_INTRO}\n\n${sections}\n`;
};

const parseSnippet = (text) => {
  const match = norm(text).match(/^test: (\S+)\nlines: (\d+)-(\d+)\n\n([\s\S]*)$/);
  if (!match) throw new Error("Invalid example snippet");
  return {
    test: match[1],
    from: Number(match[2]),
    to: Number(match[3]),
    raw: match[4].replace(/\n$/, ""),
  };
};

export const formatLines = (records) => {
  const lines = [...records].sort((a, b) => a.file.localeCompare(b.file)).map((record) =>
    JSON.stringify({ file: record.file, test: record.test, from: record.from, to: record.to, raw: record.raw }),
  );
  return lines.length ? `${lines.join("\n")}\n` : "";
};

export const parseLines = (text) => {
  const records = new Map();
  for (const line of norm(text).split("\n")) {
    if (!line.trim()) continue;
    const record = JSON.parse(line);
    records.set(record.file, record);
  }
  return records;
};

const lineStartsOf = (source) => {
  const starts = [0];
  for (let i = 0; i < source.length; i++) if (source[i] === "\n") starts.push(i + 1);
  return starts;
};

const lineAt = (starts, offset) => {
  let lo = 0;
  let hi = starts.length - 1;
  while (lo < hi) {
    const mid = (lo + hi + 1) >> 1;
    if (starts[mid] <= offset) lo = mid;
    else hi = mid - 1;
  }
  return lo + 1;
};

const skipQuoted = (source, i, quote) => {
  let j = i + 1;
  while (j < source.length) {
    if (source[j] === "\\") {
      j += 2;
      continue;
    }
    if (source[j] === "\n") return j;
    if (source[j] === quote) return j + 1;
    j++;
  }
  return source.length;
};

const skipVerbatim = (source, i) => {
  let j = source.indexOf('"', i) + 1;
  while (j < source.length) {
    if (source[j] === '"' && source[j + 1] === '"') {
      j += 2;
      continue;
    }
    if (source[j] === '"') return j + 1;
    j++;
  }
  return source.length;
};

const skipCs = (source, i) => {
  const raw = ['$@"""', '@$"""', '$"""', '@"""', '"""'].find((token) => source.startsWith(token, i));
  if (raw) {
    const end = source.indexOf('"""', i + raw.length);
    return end < 0 ? source.length : end + 3;
  }
  if (source.startsWith('$@"', i) || source.startsWith('@$"', i) || source.startsWith('@"', i)) return skipVerbatim(source, i);
  if (source[i] === '"') return skipQuoted(source, i, '"');
  if (source[i] === "'") return skipQuoted(source, i, "'");
  return null;
};

const skipRust = (source, i) => {
  let j = i;
  if (source[j] === "b" || source[j] === "c") j++;
  if (source[j] === "r") {
    let k = j + 1;
    let hashes = 0;
    while (source[k] === "#") {
      hashes++;
      k++;
    }
    if (source[k] !== '"') return null;
    const close = `"${"#".repeat(hashes)}`;
    const end = source.indexOf(close, k + 1);
    return end < 0 ? source.length : end + close.length;
  }
  if (source[j] === '"') return skipQuoted(source, j, '"');
  if (source[i] === "'") return skipQuoted(source, i, "'");
  return null;
};

const skipTs = (source, i) => {
  if (source[i] === "'" || source[i] === '"') return skipQuoted(source, i, source[i]);
  if (source[i] !== "`") return null;
  let j = i + 1;
  while (j < source.length) {
    if (source[j] === "\\") {
      j += 2;
      continue;
    }
    if (source[j] === "`") return j + 1;
    j++;
  }
  return source.length;
};

const skipString = (source, i, lang) =>
  ({ csharp: skipCs, rust: skipRust, typescript: skipTs })[lang]?.(source, i) ?? null;

const scanBraces = (source, lang) => {
  const events = [];
  let line = 1;
  let i = 0;
  while (i < source.length) {
    const c = source[i];
    if (c === "\n") {
      line++;
      i++;
      continue;
    }
    if (c === "/" && source[i + 1] === "/") {
      const nl = source.indexOf("\n", i);
      i = nl < 0 ? source.length : nl;
      continue;
    }
    if (c === "/" && source[i + 1] === "*") {
      const end = source.indexOf("*/", i + 2);
      const stop = end < 0 ? source.length : end + 2;
      line += source.slice(i, stop).split("\n").length - 1;
      i = stop;
      continue;
    }
    const stringEnd = skipString(source, i, lang);
    if (stringEnd != null && stringEnd > i) {
      line += source.slice(i, stringEnd).split("\n").length - 1;
      i = stringEnd;
      continue;
    }
    if (c === "{" || c === "}") events.push({ ch: c, at: i, line });
    i++;
  }
  return events;
};

const closeLine = (events, starts, fromLine) => {
  const start = starts[fromLine - 1] ?? 0;
  const openAt = events.find((event) => event.ch === "{" && event.at >= start);
  if (!openAt) return null;
  let depth = 0;
  for (const event of events) {
    if (event.at < openAt.at) continue;
    depth += event.ch === "{" ? 1 : -1;
    if (depth === 0) return lineAt(starts, event.at);
  }
  return null;
};

const methodName = (line) => line.match(/(\w+)\s*(?:<[^>]*>)?\s*\(/)?.[1];

const findLineSpan = (lines, sliceLines, from = 0) => {
  if (!sliceLines.length || sliceLines.length > lines.length) return -1;
  outer: for (let i = from; i <= lines.length - sliceLines.length; i++) {
    for (let j = 0; j < sliceLines.length; j++) if (lines[i + j] !== sliceLines[j]) continue outer;
    return i;
  }
  return -1;
};

const diffLines = (before, after) => {
  const n = before.length;
  const m = after.length;
  const dp = Array.from({ length: n + 1 }, () => new Array(m + 1).fill(0));
  for (let i = n - 1; i >= 0; i--)
    for (let j = m - 1; j >= 0; j--)
      dp[i][j] = before[i] === after[j] ? dp[i + 1][j + 1] + 1 : Math.max(dp[i + 1][j], dp[i][j + 1]);
  const ops = [];
  let i = 0;
  let j = 0;
  while (i < n && j < m) {
    if (before[i] === after[j]) {
      ops.push(["eq", before[i]]);
      i++;
      j++;
    } else if (dp[i + 1][j] >= dp[i][j + 1]) ops.push(["del", before[i++]]);
    else ops.push(["ins", after[j++]]);
  }
  while (i < n) ops.push(["del", before[i++]]);
  while (j < m) ops.push(["ins", after[j++]]);
  return ops;
};

const mapSlice = (oldText, newText, slice) => {
  const oldLines = norm(oldText).split("\n");
  const sliceLines = norm(slice).split("\n");
  const at = findLineSpan(oldLines, sliceLines);
  if (at < 0) return null;
  const end = at + sliceLines.length;
  const out = [];
  let o = 0;
  let started = false;
  let pending = [];
  for (const [op, line] of diffLines(oldLines, norm(newText).split("\n"))) {
    if (op === "ins") {
      if (started) pending.push(line);
      continue;
    }
    const inside = o >= at && o < end;
    if (inside) {
      if (started) out.push(...pending);
      pending = [];
      started = true;
      if (op === "eq") out.push(line);
    } else if (started) break;
    else pending = [];
    o++;
  }
  return out.join("\n");
};

export const applyEdits = (slice, edits) => {
  let next = norm(slice);
  for (const edit of edits ?? []) {
    const oldString = norm(edit.old_string ?? "");
    const newString = norm(edit.new_string ?? "");
    if (!oldString) continue;
    if (next.includes(oldString)) next = next.replaceAll(oldString, newString);
    else if (oldString.includes(next)) {
      const mapped = mapSlice(oldString, newString, next);
      if (mapped == null) return null;
      next = mapped;
    }
  }
  return next;
};

const pushRegion = (regions, source, events, starts, fromLine, declLine, name, kind) => {
  if (!name) return;
  const to = closeLine(events, starts, declLine);
  if (to == null) return;
  regions.push({ name, from: fromLine, to, kind });
};

const nextCodeLine = (lines, index) => {
  let cursor = index;
  while (cursor < lines.length && (!lines[cursor].trim() || /^\s*(?:\[|#\[)/.test(lines[cursor]))) cursor++;
  return cursor;
};

const collectCsharp = (lines, events, starts, regions, owners) => {
  for (let i = 0; i < lines.length; i++) {
    const type = lines[i].match(/\b(?:class|record|struct|interface|enum)\s+(\w+)/);
    if (type) {
      const to = closeLine(events, starts, i + 1);
      if (to != null) owners.push({ name: type[1], from: i + 1, to, kind: "type" });
    }
    if (!/^\s*\[(?:Fact|Theory)\b/.test(lines[i])) continue;
    const decl = nextCodeLine(lines, i + 1);
    pushRegion(regions, null, events, starts, i + 1, decl + 1, methodName(lines[decl] ?? ""), "test");
  }
  for (let i = 0; i < lines.length; i++) {
    if (!/^\s*(?:(?:public|private|protected|internal|static|async|override|virtual|sealed)\s+)+/.test(lines[i]))
      continue;
    if (/^\s*\[/.test(lines[i])) continue;
    pushRegion(regions, null, events, starts, i + 1, i + 1, methodName(lines[i]), "fn");
  }
};

const collectRust = (lines, events, starts, regions, owners) => {
  for (let i = 0; i < lines.length; i++) {
    const impl = lines[i].match(/^\s*impl(?:\s*<[^;{]*>)?\s+([\w:]+)(?:<[^;{]*>)?(?:\s+for\s+([\w:]+))?/);
    if (impl) {
      const name = impl[2] ? `${impl[1].split("::").pop()}For${impl[2].split("::").pop()}` : impl[1].split("::").pop();
      const to = closeLine(events, starts, i + 1);
      if (to != null) {
        owners.push({ name, from: i + 1, to, kind: "type" });
        regions.push({ name, from: i + 1, to, kind: "type" });
      }
    }
    const fn = lines[i].match(/^\s*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+(\w+)/);
    if (fn) pushRegion(regions, null, events, starts, i + 1, i + 1, fn[1], "fn");
    if (!/^\s*#\[(?:[\w:]+::)?test(?:\([^)]*\))?\]\s*$/.test(lines[i])) continue;
    const decl = nextCodeLine(lines, i + 1);
    const name = lines[decl]?.match(/^\s*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+(\w+)/)?.[1];
    pushRegion(regions, null, events, starts, i + 1, decl + 1, name, "test");
  }
};

const collectTypescript = (lines, events, starts, regions, owners) => {
  for (let i = 0; i < lines.length; i++) {
    const described = lines[i].match(/\bdescribe\(\s*(['"`])((?:\\.|(?!\1).)*)\1/);
    if (described) {
      const to = closeLine(events, starts, i + 1);
      if (to != null) owners.push({ name: described[2], from: i + 1, to, kind: "type" });
    }
    const type = lines[i].match(/(?:^|\s)class\s+(\w+)/);
    if (type) {
      const to = closeLine(events, starts, i + 1);
      if (to != null) regions.push({ name: type[1], from: i + 1, to, kind: "type" });
    }
    const fn = lines[i].match(/(?:^|\s)(?:async\s+)?function\s+(\w+)/);
    if (fn) pushRegion(regions, null, events, starts, i + 1, i + 1, fn[1], "fn");
    const test = lines[i].match(/\b(?:it|test)(?:\.(?:only|skip|concurrent))?\(\s*(['"`])((?:\\.|(?!\1).)*)\1/);
    if (test) pushRegion(regions, null, events, starts, i + 1, i + 1, slug(test[2]), "test");
  }
};

const uniquify = (regions, owners) => {
  const kept = regions.filter(
    (region) => region.kind === "test" || !regions.some((other) => other.kind === "test" && other.to === region.to),
  );
  const counts = new Map();
  for (const region of kept) counts.set(region.name, (counts.get(region.name) ?? 0) + 1);
  const seen = new Map();
  return kept.map((region) => {
      if (counts.get(region.name) === 1) return region;
      const owner = owners
        .filter((item) => item.from <= region.from && item.to >= region.to && item.name !== region.name)
        .sort((a, b) => a.to - a.from - (b.to - b.from))[0];
      const n = (seen.get(region.name) ?? 0) + 1;
      seen.set(region.name, n);
      const name = owner ? `${slug(owner.name)}-${region.name}` : `${region.name}-${n}`;
      return { ...region, name };
    });
};

export const findRegions = (source, lang) => {
  const text = norm(source);
  const lines = text.split("\n");
  const starts = lineStartsOf(text);
  const events = scanBraces(text, lang);
  const found = [];
  const owners = [];
  ({
    csharp: collectCsharp,
    rust: collectRust,
    typescript: collectTypescript,
  })[lang]?.(lines, events, starts, found, owners);
  const regions = uniquify(found, owners);
  for (const owner of owners)
    if (!regions.some((region) => region.kind === "type" && region.from === owner.from && region.to === owner.to))
      if (lang === "csharp") regions.push(owner);
  return regions;
};

const overlapCount = (region, from, to) => {
  const start = Math.max(region.from, from);
  const end = Math.min(region.to, to);
  return end >= start ? end - start + 1 : 0;
};

export const regionForRange = (regions, from, to) => {
  const covering = (list) =>
    list
      .filter((region) => region.from <= from && region.to >= to)
      .sort((a, b) => a.to - a.from - (b.to - b.from))[0];
  const bestOverlap = (list) => {
    const ranked = list
      .filter((region) => overlapCount(region, from, to) > 0)
      .sort(
        (a, b) =>
          overlapCount(b, from, to) - overlapCount(a, from, to) || a.to - a.from - (b.to - b.from),
      );
    const [best, second] = ranked;
    if (!best) return null;
    if (
      second &&
      overlapCount(second, from, to) > 1 &&
      overlapCount(second, from, to) * 2 >= overlapCount(best, from, to)
    )
      throw new Error(`lines ${from}-${to} span ${best.name} and ${second.name}`);
    return best;
  };
  const tests = regions.filter((region) => region.kind === "test");
  return (
    covering(tests) ??
    bestOverlap(tests) ??
    covering(regions) ??
    bestOverlap(regions) ??
    (() => {
      throw new Error(`lines ${from}-${to} are not inside a test, type, or function`);
    })()
  );
};

export const regionByName = (regions, name) => regions.find((region) => region.name === name) ?? null;

export const readSnippet = (heading, lang, spec, base = root) => {
  const pointer = parsePointer(spec);
  if (!pointer.name) throw new Error(`${spec}: run npm run docs:examples`);
  const rel = snippetFile({ heading, sourcePath: pointer.path, name: pointer.name, lang });
  let record = null;
  try {
    record = parseLines(readFileSync(join(base, LINES), "utf8")).get(rel) ?? null;
  } catch {
    record = null;
  }
  if (!record) throw new Error(`${spec}: run npm run docs:examples`);
  const file = join(base, rel);
  return {
    path: pointer.path,
    from: record.from,
    to: record.to,
    body: readFileSync(file, "utf8").replace(/\n$/, ""),
    file,
    linesFile: join(base, LINES),
  };
};

export const locateSlice = (source, region, slice, hint) => {
  const lines = norm(source).split("\n");
  const body = lines.slice(region.from - 1, region.to);
  const sliceLines = norm(slice).split("\n");
  const found = [];
  let at = findLineSpan(body, sliceLines);
  while (at >= 0) {
    found.push({ from: region.from + at, to: region.from + at + sliceLines.length - 1 });
    at = findLineSpan(body, sliceLines, at + 1);
  }
  if (!found.length) return null;
  if (found.length === 1) return found[0];
  if (hint?.from) {
    const exact = found.find((item) => item.from === hint.from && item.to === hint.to);
    if (exact) return exact;
    return found.sort((a, b) => Math.abs(a.from - hint.from) - Math.abs(b.from - hint.from))[0];
  }
  return { ambiguous: true };
};

const readLines = (source, from, to) => norm(source).split("\n").slice(from - 1, to).join("\n");

const readCatalog = (base) => {
  try {
    return parseLines(readFileSync(join(base, LINES), "utf8"));
  } catch {
    return new Map();
  }
};

const loadStored = (catalog, rel, path) => {
  const record = catalog.get(rel);
  if (record) return { from: record.from, to: record.to, raw: record.raw };
  try {
    return parseSnippet(readFileSync(path, "utf8"));
  } catch {
    return null;
  }
};

const readSample = (path) => {
  try {
    const text = readFileSync(path, "utf8");
    return text.startsWith("test: ") ? null : text.replace(/\n$/, "");
  } catch {
    return null;
  }
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

const samplesIn = (markdown) => {
  const samples = [];
  let offset = 0;
  for (const part of markdown.split(DOC_FENCE)) {
    if (!part.startsWith("~~~~")) {
      const lines = part.split("\n");
      let cursor = offset;
      let heading = null;
      let inCode = false;
      let skipHeading = 0;
      for (const line of lines) {
        if (line.startsWith("```")) {
          if (!inCode) {
            const match = line.match(/^```([a-zA-Z0-9_+-]+)\s+(\S+#\S+)\s*$/);
            if (match)
              samples.push({
                heading,
                lang: match[1],
                spec: match[2],
                index: cursor + line.indexOf(match[2]),
              });
          }
          inCode = !inCode;
        } else if (!inCode && (line.startsWith("::: prose") || line.startsWith("::: reference"))) {
          skipHeading++;
        } else if (!inCode && line.trim() === ":::" && skipHeading) {
          skipHeading--;
        } else if (!inCode && !skipHeading) {
          const title = line.match(/^###\s+(.+?)\s*$/);
          if (title) heading = title[1];
        }
        cursor += line.length + 1;
      }
    }
    offset += part.length;
  }
  return samples;
};

const relPath = (file) => relative(root, resolve(file)).split("\\").join("/");

const headingsIn = (base) => {
  const headings = [];
  for (const doc of walkMd(join(base, "docs"))) {
    const docRel = relative(base, resolve(doc)).split("\\").join("/");
    if (docRel === EXAMPLES_PAGE) continue;
    const seen = new Set();
    for (const sample of samplesIn(readFileSync(doc, "utf8"))) {
      if (!sample.heading || seen.has(sample.heading)) continue;
      seen.add(sample.heading);
      headings.push({ doc: docRel, title: sample.heading });
    }
  }
  return headings;
};

const exampleDirectories = (base) =>
  new Set(
    readdirSync(join(base, "docs/examples"), { withFileTypes: true })
      .filter((entry) => entry.isDirectory())
      .map((entry) => entry.name),
  );

const directoryErrorsFor = (headings, directories) => {
  const slugs = new Set(headings.map((heading) => slug(heading.title)));
  const errors = [...directories]
    .filter((dir) => !slugs.has(dir))
    .sort()
    .map((dir) => `docs/examples/${dir}: no ### heading`);
  const seen = new Set();
  for (const heading of headings) {
    const key = slug(heading.title);
    if (seen.has(key) || directories.has(key)) continue;
    seen.add(key);
    errors.push(`${heading.doc}: ${heading.title} has no docs/examples/${key} directory`);
  }
  return errors;
};

const readOptional = (path) => {
  try {
    return readFileSync(path, "utf8");
  } catch {
    return null;
  }
};

export const syncExamples = ({ files, check = false, edits = null, base = root } = {}) => {
  const only = files?.length ? new Set(files.map((file) => relPath(file))) : null;
  const errors = [];
  const catalog = readCatalog(base);
  const sampleWrites = new Map();
  const lineRecords = new Map();
  const docEdits = new Map();
  let count = 0;
  for (const doc of walkMd(join(base, "docs"))) {
    const markdown = readFileSync(doc, "utf8");
    const docRel = relPath(doc);
    for (const sample of samplesIn(markdown)) {
      if (!sample.heading) {
        errors.push(`${docRel}: example ${sample.spec} has no ### heading`);
        continue;
      }
      let pointer;
      try {
        pointer = parsePointer(sample.spec);
      } catch (error) {
        errors.push(`${docRel}: ${error.message}`);
        continue;
      }
      if (only && !only.has(pointer.path)) continue;
      count++;
      const where = `${docRel} (${sample.heading}) ${sample.spec}`;
      const lang = normalizeLang(sample.lang);
      const sourcePath = join(base, pointer.path);
      let source;
      try {
        source = readFileSync(sourcePath, "utf8");
      } catch {
        errors.push(`${where}: missing ${pointer.path}`);
        continue;
      }
      const regions = findRegions(source, lang);
      let region;
      let body;
      let from;
      let to;
      try {
        if (pointer.name) {
          region = regionByName(regions, pointer.name);
          if (!region) throw new Error(`no region ${pointer.name}`);
          const relSnippet = snippetFile({
            heading: sample.heading,
            sourcePath: pointer.path,
            name: pointer.name,
            lang,
          });
          const snippetPath = join(base, relSnippet);
          const stored = loadStored(catalog, relSnippet, snippetPath);
          if (!stored) {
            if (check) throw new Error("missing snippet");
            body = readLines(source, region.from, region.to);
            from = region.from;
            to = region.to;
          } else {
            let found = locateSlice(source, region, stored.raw, stored);
            let nextBody = stored.raw;
            if (!found && edits?.length) {
              const edited = applyEdits(stored.raw, edits);
              if (edited && edited !== stored.raw) {
                found = locateSlice(source, region, edited);
                nextBody = edited;
              }
            }
            if (!found) throw new Error(`slice not in ${pointer.name}`);
            if (found.ambiguous) throw new Error(`slice is ambiguous in ${pointer.name}`);
            body = nextBody;
            from = found.from;
            to = found.to;
            if (
              check &&
              (stored.from !== from ||
                stored.to !== to ||
                stored.raw !== body ||
                readSample(snippetPath) !== exampleCode(body))
            )
              throw new Error(`drifted to ${from}-${to}`);
          }
        } else {
          region = regionForRange(regions, pointer.from, pointer.to);
          from = Math.max(pointer.from, region.from);
          to = Math.min(pointer.to, region.to);
          if (from > to) throw new Error(`lines ${pointer.from}-${pointer.to} miss ${region.name}`);
          body = readLines(source, from, to);
          if (check) throw new Error("still uses a line range");
        }
      } catch (error) {
        errors.push(`${where}: ${error.message}`);
        continue;
      }
      const name = region.name;
      const test = `${pointer.path}#${name}`;
      const relSnippet = snippetFile({ heading: sample.heading, sourcePath: pointer.path, name, lang });
      const record = { file: relSnippet, test, from, to, raw: body };
      const previous = lineRecords.get(relSnippet);
      if (previous && JSON.stringify(previous) !== JSON.stringify(record))
        errors.push(`${where}: snippet ${relSnippet} already holds a different slice`);
      else {
        lineRecords.set(relSnippet, record);
        sampleWrites.set(relSnippet, `${exampleCode(body)}\n`);
      }
      if (sample.spec !== test)
        docEdits.set(doc, [...(docEdits.get(doc) ?? []), { index: sample.index, length: sample.spec.length, text: test }]);
    }
  }
  if (errors.length) return { errors, count, changed: 0 };
  const headings = headingsIn(base);
  const pageText = renderExamplesPage({
    pages: parseToc(readFileSync(join(base, "docs/toc.yaml"), "utf8")).pages,
    headings,
  });
  if (check) {
    const pageErrors = directoryErrorsFor(headings, exampleDirectories(base));
    if (readOptional(join(base, EXAMPLES_PAGE)) !== pageText)
      pageErrors.push("docs/examples.md is stale. Run npm run docs:examples.");
    return { errors: pageErrors, count, changed: 0 };
  }
  let changed = 0;
  for (const [relSnippet, text] of sampleWrites) {
    const path = join(base, relSnippet);
    let current = null;
    try {
      current = readFileSync(path, "utf8");
    } catch {
      current = null;
    }
    if (current === text) continue;
    mkdirSync(dirname(path), { recursive: true });
    writeFileSync(path, text);
    changed++;
  }
  const nextCatalog = only ? new Map(catalog) : new Map();
  for (const record of lineRecords.values()) nextCatalog.set(record.file, record);
  const linesText = formatLines(nextCatalog.values());
  const linesFile = join(base, LINES);
  let currentLines = null;
  try {
    currentLines = readFileSync(linesFile, "utf8");
  } catch {
    currentLines = null;
  }
  if (currentLines !== linesText) {
    mkdirSync(dirname(linesFile), { recursive: true });
    writeFileSync(linesFile, linesText);
    changed++;
  }
  for (const [doc, editsOfDoc] of docEdits) {
    let text = readFileSync(doc, "utf8");
    for (const edit of editsOfDoc.sort((a, b) => b.index - a.index))
      text = text.slice(0, edit.index) + edit.text + text.slice(edit.index + edit.length);
    writeFileSync(doc, text);
    changed++;
  }
  const pageErrors = directoryErrorsFor(headings, exampleDirectories(base));
  if (pageErrors.length) return { errors: pageErrors, count, changed };
  const pagePath = join(base, EXAMPLES_PAGE);
  if (readOptional(pagePath) !== pageText) {
    writeFileSync(pagePath, pageText);
    changed++;
  }
  return { errors, count, changed };
};

const run = () => {
  const args = process.argv.slice(2);
  const check = args.includes("--check");
  const files = args.filter((arg) => arg !== "--check");
  const { errors, count, changed } = syncExamples({ files, check });
  if (!onlyMatched(files, count) && files.length) return 0;
  if (errors.length) {
    for (const error of errors) console.error(error);
    return 1;
  }
  if (!check) console.log(`synced ${count} examples${changed ? `, updated ${changed}` : ""}`);
  return 0;
};

const onlyMatched = (files, count) => !files.length || count > 0;

const isDirect =
  process.argv[1] && import.meta.url === pathToFileURL(resolve(process.argv[1])).href;

if (isDirect) process.exit(run());
