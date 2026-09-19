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

const balanced = (text, open, close) => {
  const start = text.indexOf(open);
  if (start < 0) return null;
  let depth = 0;
  for (let i = start; i < text.length; i++) {
    if (text[i] === open) depth++;
    else if (text[i] === close && --depth === 0) return text.slice(start + 1, i);
  }
  return null;
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
  /\[Fact\]|#\[test\]|\bit\(|public void |fn \w+\(/.test(code) ? (balanced(code, "{", "}") ?? code) : code;

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
  while (lines[0] && /^(?:(?:DatasourceFactory\.)?reset|clear_env)\(\);$/.test(lines[0].trim()))
    lines.shift();
  return lines.join("\n");
};

export const pertinent = (code) =>
  dedent(unwrapThrows(stripTrailingValueAsserts(stripHarness(extractBody(code)))));

export const readRange = (spec, base = root) => {
  const { path, from, to } = parseRange(spec);
  const lines = readFileSync(join(base, path), "utf8").split(/\r?\n/);
  if (to > lines.length) throw new Error(`${spec}: file has ${lines.length} lines`);
  const slice = lines.slice(from - 1, to);
  if (slice.every((line) => !line.trim())) throw new Error(`${spec}: range is empty`);
  return { path, from, to, code: pertinent(dedent(slice)) };
};

export const blobUrl = ({ path, from, to }) => `${REPO}/blob/${BRANCH}/${path}#L${from}-L${to}`;
