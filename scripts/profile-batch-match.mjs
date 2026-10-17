import { createRequire } from "node:module";
import { performance } from "node:perf_hooks";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const require = createRequire(
  resolve(dirname(fileURLToPath(import.meta.url)), "../typescript/providers/sqlite/package.json"),
);
const Database = require("better-sqlite3");

const splitStatements = (sql) => sql.split(";").map((part) => part.trim()).filter(Boolean);

const statementUses = (sql, name) => {
  const prefixes = ["$", ":", "@"];
  const bare = name.replace(/^[$:@]/, "");
  return prefixes.some((prefix) => {
    const token = `${prefix}${bare}`;
    let from = 0;
    while (from <= sql.length) {
      const at = sql.indexOf(token, from);
      if (at < 0) return false;
      const next = sql[at + token.length];
      if (next === undefined || !/[0-9A-Za-z_]/.test(next)) return true;
      from = at + token.length;
    }
    return false;
  });
};

const extract = (sql) => {
  const names = new Set();
  for (let i = 0; i < sql.length; i++) {
    const ch = sql[i];
    if (ch !== "$" && ch !== ":" && ch !== "@") continue;
    if (ch === "$" && sql[i + 1] === "$") continue;
    let end = i + 1;
    while (end < sql.length && /[0-9A-Za-z_]/.test(sql[end])) end += 1;
    if (end > i + 1) names.add(sql.slice(i + 1, end));
    i = end - 1;
  }
  return names;
};

const cols = ["id", "username", "password", "is_enabled", "created", "updated"];

const buildJoined = (n) => {
  const texts = [];
  const parameters = [];
  for (let i = 0; i < n; i++) {
    texts.push(
      `INSERT INTO users (${cols.join(", ")}) VALUES (${cols.map((c) => `$${c}__${i}`).join(", ")})`,
    );
    for (const col of cols) parameters.push({ name: `${col}__${i}`, value: i });
  }
  return { text: texts.join("; "), parameters };
};

const buildMulti = (n) => {
  const parameters = [];
  const values = [];
  for (let i = 0; i < n; i++) {
    values.push(`(${cols.map((c) => `$${c}__${i}`).join(", ")})`);
    for (const col of cols) parameters.push({ name: `${col}__${i}`, value: i });
  }
  return {
    text: `INSERT INTO users (${cols.join(", ")}) VALUES ${values.join(", ")}`,
    parameters,
  };
};

const matchOld = (text, parameters) => {
  const parts = splitStatements(text);
  return parts.map((part) => parameters.filter((p) => statementUses(part, p.name)));
};

const matchNew = (text, parameters) => {
  const parts = splitStatements(text);
  const byName = new Map(parameters.map((p) => [p.name, p]));
  return parts.map((part) => [...extract(part)].map((name) => byName.get(name)).filter(Boolean));
};

const exec = (sql, parameters) => {
  const db = new Database(":memory:");
  db.exec(
    "CREATE TABLE users (id INTEGER, username TEXT, password TEXT, is_enabled INTEGER, created TEXT, updated TEXT)",
  );
  const parts = splitStatements(sql);
  for (const part of parts) {
    const used = Object.fromEntries(
      parameters.filter((p) => statementUses(part, p.name)).map((p) => [p.name, p.value]),
    );
    db.prepare(part).run(used);
  }
  db.close();
};

const time = (label, fn) => {
  const started = performance.now();
  fn();
  return { label, ms: performance.now() - started };
};

const sizes = [100, 500, 1000];
for (const n of sizes) {
  const joined = buildJoined(n);
  const multi = buildMulti(n);
  const rows = [
    time(`joined-build-${n}`, () => buildJoined(n)),
    time(`joined-match-old-${n}`, () => matchOld(joined.text, joined.parameters)),
    time(`joined-match-new-${n}`, () => matchNew(joined.text, joined.parameters)),
    time(`joined-exec-${n}`, () => exec(joined.text, joined.parameters)),
    time(`multi-build-${n}`, () => buildMulti(n)),
    time(`multi-match-old-${n}`, () => matchOld(multi.text, multi.parameters)),
    time(`multi-match-new-${n}`, () => matchNew(multi.text, multi.parameters)),
  ];
  for (const row of rows) process.stdout.write(`${row.label}\t${row.ms.toFixed(1)} ms\n`);
  process.stdout.write("\n");
}
