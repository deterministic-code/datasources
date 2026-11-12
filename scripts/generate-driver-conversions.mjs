import { existsSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const outPath = join(root, "docs/DriverConversions.yaml");
const banner = "# Generated from C# Conversion[] tables. Run npm run docs:drivers.\n";

const DRIVERS = ["sqlite", "mysql", "postgres", "sqlserver", "oracle"];
const TABLES = ["typeConverter", "rowTypeConverters", "defaultValueConverter"];
const ARRAYS = {
  TypeConverter: "typeConverter",
  ReverseTypeConverter: "typeConverter",
  RowTypeConverters: "rowTypeConverters",
  ReverseRowTypeConverters: "rowTypeConverters",
  DefaultValueConverter: "defaultValueConverter",
  ReverseDefaultValueConverter: "defaultValueConverter",
};

const matching = (src, open) => {
  const pairs = { "(": ")", "[": "]", "{": "}" };
  const stack = [src[open]];
  let quote = "";
  for (let i = open + 1; i < src.length; i++) {
    const c = src[i];
    if (quote) {
      if (c === "\\" && quote === '"') {
        i++;
        continue;
      }
      if (c === quote) quote = "";
      continue;
    }
    if (c === '"' || c === "'") {
      quote = c;
      continue;
    }
    if (c in pairs) {
      stack.push(c);
      continue;
    }
    if (c === ")" || c === "]" || c === "}") {
      if (pairs[stack.at(-1)] !== c) continue;
      stack.pop();
      if (stack.length === 0) return i;
    }
  }
  return -1;
};

const squeeze = (text) => {
  let quote = "";
  let out = "";
  for (let i = 0; i < text.length; i++) {
    const c = text[i];
    if (quote) {
      out += c;
      if (c === "\\" && quote === '"') {
        out += text[++i] ?? "";
        continue;
      }
      if (c === quote) quote = "";
      continue;
    }
    if (c === '"' || c === "'") {
      quote = c;
      out += c;
      continue;
    }
    if (/\s/.test(c)) {
      if (out.at(-1) !== " ") out += " ";
      continue;
    }
    out += c;
  }
  return out.trim();
};

const topArgs = (inner) => {
  const args = [];
  let start = 0;
  let depth = 0;
  const openers = { "(": ")", "[": "]", "{": "}" };
  const closers = { ")": "(", "]": "[", "}": "{" };
  let quote = "";
  for (let i = 0; i < inner.length; i++) {
    const c = inner[i];
    if (quote) {
      if (c === "\\" && quote === '"') {
        i++;
        continue;
      }
      if (c === quote) quote = "";
      continue;
    }
    if (c === '"' || c === "'") {
      quote = c;
      continue;
    }
    if (c in openers) depth++;
    else if (c in closers) depth--;
    else if (c === "," && depth === 0) {
      args.push(inner.slice(start, i).trim());
      start = i + 1;
    }
  }
  args.push(inner.slice(start).trim());
  return args;
};

const unquote = (text) => {
  const t = squeeze(text);
  if (t.length >= 2 && t[0] === '"' && t.at(-1) === '"') {
    try {
      return JSON.parse(t);
    } catch {
      return t.slice(1, -1);
    }
  }
  return t;
};

const cell = (text) => {
  const t = squeeze(text);
  const type = t.match(/^typeof\((\w+)\)$/);
  if (type) return type[1];
  if (t === "null") return "null";
  return t;
};

const entries = (inner) => {
  const rows = [];
  let i = 0;
  while (i < inner.length) {
    const start = inner.indexOf("new(", i);
    if (start < 0) break;
    const open = start + 3;
    const close = matching(inner, open);
    if (close < 0) break;
    rows.push(topArgs(inner.slice(open + 1, close)).map(squeeze));
    i = close + 1;
  }
  return rows;
};

const field = (table, text) =>
  table === "typeConverter" || table === "defaultValueConverter"
    ? squeeze(text).startsWith('"')
      ? unquote(text)
      : cell(text)
    : cell(text);

const rowOf = (table, args) => {
  const [type, from, to] = args;
  return { type: unquote(type), from: field(table, from), to: field(table, to) };
};

const add = (drivers, driver, table, row) => {
  drivers[driver] ??= { typeConverter: [], rowTypeConverters: [], defaultValueConverter: [] };
  drivers[driver][table].push(row);
};

const scan = (src, driver, drivers) => {
  for (const [name, table] of Object.entries(ARRAYS)) {
    const needle = `Conversion[] ${name}`;
    const at = src.indexOf(needle);
    if (at < 0) continue;
    const open = src.indexOf("[", at + needle.length);
    const close = matching(src, open);
    if (close < 0) continue;
    for (const args of entries(src.slice(open + 1, close))) {
      if (args.length !== 3) continue;
      add(drivers, driver, table, rowOf(table, args));
    }
  }
};

const folderOf = (id) => id[0].toUpperCase() + id.slice(1);

const filesOf = (id) => {
  const dir = join(root, "csharp/Providers", `${folderOf(id)}.Tests`);
  return readdirSync(dir)
    .filter((name) => name.endsWith("DriverConvertersTests.cs"))
    .map((name) => join(dir, name));
};

const yamlScalar = (value) => JSON.stringify(value);

const dump = (drivers) => {
  let text = banner;
  for (const id of DRIVERS) {
    const tables = drivers[id] ?? { typeConverter: [], rowTypeConverters: [], defaultValueConverter: [] };
    text += `\n${id}:\n`;
    for (const key of TABLES) {
      const rows = tables[key];
      if (!rows.length) {
        text += `  ${key}: []\n`;
        continue;
      }
      text += `  ${key}:\n`;
      for (const row of rows) {
        const fields = [
          ...(row.type ? [`type: ${yamlScalar(row.type)}`] : []),
          `from: ${yamlScalar(row.from)}`,
          `to: ${yamlScalar(row.to)}`,
        ];
        text += `    - ${fields[0]}\n`;
        for (const field of fields.slice(1)) text += `      ${field}\n`;
      }
    }
  }
  return text;
};

export const generate = () => {
  const drivers = {};
  for (const id of DRIVERS) {
    for (const path of filesOf(id)) scan(readFileSync(path, "utf8"), id, drivers);
  }
  return dump(drivers);
};

const isMain = process.argv[1] && fileURLToPath(import.meta.url) === process.argv[1];
if (isMain) {
  const next = generate();
  if (process.argv.includes("--check")) {
    const current = existsSync(outPath) ? readFileSync(outPath, "utf8") : "";
    if (current !== next) {
      console.error("docs/DriverConversions.yaml is stale. Run npm run docs:drivers.");
      process.exit(1);
    }
    process.exit(0);
  }
  writeFileSync(outPath, next);
  console.log("Wrote docs/DriverConversions.yaml");
}
