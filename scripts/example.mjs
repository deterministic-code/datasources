import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const RANGE = /^(.+)#L(\d+)-L(\d+)$/;
const root = join(dirname(fileURLToPath(import.meta.url)), "..");

export const REPO = "https://github.com/deterministic-code/datasources";
export const BRANCH = "main";
export const SOURCE_ICON = "⟨/⟩";

export const parseRange = (spec) => {
  const match = spec.trim().match(RANGE);
  if (!match) throw new Error(`Invalid example range: ${spec}`);
  const from = Number(match[2]);
  const to = Number(match[3]);
  if (from < 1 || to < from) throw new Error(`Invalid example range: ${spec}`);
  return { path: match[1], from, to };
};

const dedent = (lines) => {
  const list = Array.isArray(lines) ? lines : lines.split("\n");
  const widths = list.filter((line) => line.trim()).map((line) => line.match(/^[ \t]*/)[0].length);
  const pad = widths.length ? Math.min(...widths) : 0;
  return list.map((line) => line.slice(pad)).join("\n").replace(/^\n+|\n+$/g, "");
};

const ATTRIBUTE = /^\s*(?:\[[^\]]*\]|#\[[^\]]*\])\s*$/;
const SIGNATURE =
  /^(?:it|test)(?:\.\w+)?\s*\(|^(?:(?:export|public|private|protected|internal|static|async|override|virtual|sealed|new|unsafe)\s+)*(?:void|Task)\b.*\(|^(?:(?:pub(?:\([^)]*\))?\s+|async\s+|unsafe\s+|const\s+)*)fn\s+\w+/;

const skipQuoted = (text, i, quote) => {
  let j = i + 1;
  while (j < text.length) {
    if (text[j] === "\\") {
      j += 2;
      continue;
    }
    if (text[j] === "\n") return null;
    if (text[j] === quote) return j + 1;
    j++;
  }
  return null;
};

const skipCsRaw = (text, i) => {
  const token = ['$@"""', '@$"""', '$"""', '@"""', '"""'].find((item) => text.startsWith(item, i));
  if (token) {
    const end = text.indexOf('"""', i + token.length);
    return end < 0 ? text.length : end + 3;
  }
  if (text.startsWith('$@"', i) || text.startsWith('@$"', i) || text.startsWith('@"', i) || text.startsWith('$"', i)) {
    let j = text.indexOf('"', i) + 1;
    while (j < text.length) {
      if (text[j] === '"' && text[j + 1] === '"') {
        j += 2;
        continue;
      }
      if (text[j] === '"') return j + 1;
      j++;
    }
    return text.length;
  }
  return null;
};

const skipRustRaw = (text, i) => {
  let j = i;
  if (text[j] === "b" || text[j] === "c") j++;
  if (text[j] !== "r") return null;
  j++;
  let hashes = 0;
  while (text[j] === "#") {
    hashes++;
    j++;
  }
  if (text[j] !== '"') return null;
  const end = `"${"#".repeat(hashes)}`;
  const at = text.indexOf(end, j + 1);
  return at < 0 ? text.length : at + end.length;
};

const skipTemplate = (text, i) => {
  let j = i + 1;
  while (j < text.length) {
    if (text[j] === "\\") {
      j += 2;
      continue;
    }
    if (text[j] === "`") return j + 1;
    if (text[j] === "$" && text[j + 1] === "{") {
      j = closeBrace(text, j + 1);
      continue;
    }
    j++;
  }
  return text.length;
};

const skipAtom = (text, i) => {
  if (text.startsWith("//", i)) {
    const nl = text.indexOf("\n", i);
    return nl < 0 ? text.length : nl + 1;
  }
  if (text.startsWith("/*", i)) {
    const end = text.indexOf("*/", i + 2);
    return end < 0 ? text.length : end + 2;
  }
  return (
    skipCsRaw(text, i) ??
    skipRustRaw(text, i) ??
    (text[i] === "'" || text[i] === '"' ? skipQuoted(text, i, text[i]) : null) ??
    (text[i] === "`" ? skipTemplate(text, i) : null)
  );
};

const closeBrace = (text, openAt) => {
  let depth = 1;
  let j = openAt + 1;
  while (j < text.length) {
    const skipped = skipAtom(text, j);
    if (skipped != null) {
      j = skipped;
      continue;
    }
    if (text[j] === "{") depth++;
    else if (text[j] === "}" && --depth === 0) return j + 1;
    j++;
  }
  return text.length;
};

const interior = (text) => {
  let i = 0;
  let depth = 0;
  let start = -1;
  while (i < text.length) {
    const skipped = skipAtom(text, i);
    if (skipped != null) {
      i = skipped;
      continue;
    }
    if (text[i] === "{") {
      if (depth === 0) start = i + 1;
      depth++;
    } else if (text[i] === "}" && depth > 0 && --depth === 0 && start >= 0) return text.slice(start, i);
    i++;
  }
  return start < 0 ? null : text.slice(start);
};

const leadingWrapper = (code) => {
  const lines = code.split("\n");
  let i = 0;
  while (i < lines.length && (!lines[i].trim() || ATTRIBUTE.test(lines[i]))) i++;
  const line = (lines[i] ?? "").replace(/^(?:\s*(?:\[[^\]]*\]|#\[[^\]]*\])\s*)+/, "").trim();
  return SIGNATURE.test(line);
};

const parseTo = (text, stop) => {
  let depth = 0;
  for (let i = 0; i < text.length; i++) {
    const c = text[i];
    if (c === "(" || c === "[" || c === "{") depth++;
    else if (c === ")" || c === "]" || c === "}") {
      if (depth === 0) return text.slice(0, i).trim();
      depth--;
    } else if (stop(c, depth)) return text.slice(0, i).trim();
  }
  return text.trim();
};

const VALUE_ASSERT =
  /^(Assert\.(Equal|True|False|Same|NotSame|Empty|NotEmpty|Null|NotNull|StartsWith|Contains|IsType)|assert_eq!|assert_ne!|expect\(.+\)\.(toBe|toEqual|toStrictEqual))/;

const extractBody = (code) =>
  /^(?:pub(?:\([^)]*\))? +)?impl\b/.test(code.trim()) || !leadingWrapper(code) ? code : (interior(code) ?? "");

const stripTrailingValueAsserts = (body) => {
  const lines = body.split("\n");
  while (lines.length) {
    let end = lines.length - 1;
    while (end >= 0 && !lines[end].trim()) end--;
    if (end < 0) break;
    let start = end;
    while (start > 0 && !VALUE_ASSERT.test(lines[start].trim())) start--;
    if (!VALUE_ASSERT.test(lines[start].trim())) break;
    lines.length = start;
  }
  return lines.join("\n");
};

const statement = (expr) => (expr.endsWith(";") ? expr : `${expr};`);

const unwrapThrows = (body) => {
  const text = body.trim();
  const cs = text.match(/^(?:var \w+ = )?Assert\.Throws<[^>]+>\(\s*\(\)\s*=>\s*/);
  if (cs) return statement(parseTo(text.slice(cs[0].length), () => false));
  const ts = text.match(/^expect\(\s*\(\)\s*=>\s*/);
  if (ts && /\.toThrow\(/.test(text)) return statement(parseTo(text.slice(ts[0].length), () => false));
  const rs = text.match(/^assert!\(\s*matches!\(\s*/);
  if (rs) return statement(parseTo(text.slice(rs[0].length), (c, depth) => c === "," && depth === 0));
  return body;
};

const stripHarness = (body) => {
  const lines = body.replace(/^\n+/, "").split("\n");
  while (
    lines[0] &&
    /^(?:(?:DatasourceFactory\.)?reset|clear_env|arrange)\(\);$|^let _guard = super::env_lock\(\);$/.test(
      lines[0].trim(),
    )
  )
    lines.shift();
  return lines.join("\n");
};

const OPEN_CALL =
  /^(?:(?:await using|using)\s+)?(?:var|const|let)\s+\w+\s*=\s*(?:await\s+)?(?:Open|open)\s*\(/;
const APPLY_CALL =
  /^(?:await\s+)?(?:[\w.]+\.)?(?:[\w]+::)?(?:SchemaSpec\.Apply|applySchema|apply_schema)\s*\(/;

const skipLineString = (line, i) => {
  const quote = line[i];
  if (quote !== '"' && quote !== "'" && quote !== "`") return null;
  let j = i + 1;
  while (j < line.length) {
    if (line[j] === "\\") {
      j += 2;
      continue;
    }
    if (line[j] === quote) return j + 1;
    j++;
  }
  return line.length;
};

const statementEnd = (lines, start) => {
  let depth = 0;
  for (let i = start; i < lines.length; i++) {
    const line = lines[i];
    for (let j = 0; j < line.length; j++) {
      const skipped = skipLineString(line, j);
      if (skipped != null) {
        j = skipped - 1;
        continue;
      }
      const c = line[j];
      if (c === "(" || c === "[" || c === "{") depth++;
      else if (c === ")" || c === "]" || c === "}") depth = Math.max(0, depth - 1);
      else if (c === ";" && depth === 0) return i + 1;
    }
  }
  return lines.length;
};

export const stripSchema = (body) => {
  const lines = body.replace(/^\n+/, "").split("\n");
  const kept = [];
  let schema = null;
  for (let i = 0; i < lines.length; ) {
    const trimmed = lines[i].trim();
    if (OPEN_CALL.test(trimmed) || APPLY_CALL.test(trimmed)) {
      const end = statementEnd(lines, i);
      if (APPLY_CALL.test(trimmed)) {
        const name = lines.slice(i, end).join("\n").match(/["']([A-Za-z0-9-]+)["']/);
        if (name) schema = name[1];
      }
      i = end;
      continue;
    }
    kept.push(lines[i]);
    i++;
  }
  return { body: kept.join("\n"), schema };
};

export const pertinent = (code) =>
  dedent(unwrapThrows(stripTrailingValueAsserts(stripSchema(stripHarness(extractBody(code))).body)));

export const exampleSchema = (code) =>
  stripSchema(stripHarness(extractBody(code))).schema;

export const exampleCode = (slice) => pertinent(dedent(Array.isArray(slice) ? slice.join("\n") : slice));

export const readRange = (spec, base = root) => {
  const { path, from, to } = parseRange(spec);
  const lines = readFileSync(join(base, path), "utf8").split(/\r?\n/);
  if (to > lines.length) throw new Error(`${spec}: file has ${lines.length} lines`);
  const slice = lines.slice(from - 1, to);
  if (slice.every((line) => !line.trim())) throw new Error(`${spec}: range is empty`);
  return { path, from, to, code: exampleCode(slice) };
};

export const blobUrl = ({ path, from, to }) => `${REPO}/blob/${BRANCH}/${path}#L${from}-L${to}`;

export const sourceHref = ({ path, from, to }) =>
  `/view-source?path=${encodeURIComponent(path)}&from=${from}&to=${to}`;
