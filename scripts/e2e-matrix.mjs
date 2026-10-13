import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { dialectTranscript } from "./e2e-matrix-normalize.mjs";

const root = dirname(fileURLToPath(new URL(".", import.meta.url)));
const artifacts = join(root, "artifacts/e2e-matrix");
const composeFile = join(root, "docker/e2e-matrix/compose.yml");
const project = "datasources-e2e-matrix";
const dialects = ["typescript", "rust", "csharp"];
const providers = ["sqlite", "mysql", "postgres"];
const inDocker = process.env.E2E_MATRIX_IN_DOCKER === "1";
const samples = [
  { name: "user-roles-simple-cli", sql: false },
  { name: "contacts-crud-enriched-medium-rest", sql: true },
];

const mysqlUrl = () =>
  process.env.MYSQL_URL || (inDocker ? "mysql://ds:ds@mysql:3306/datasources" : "mysql://ds:ds@127.0.0.1:13306/datasources");
const postgresUrl = () =>
  process.env.POSTGRES_URL ||
  (inDocker ? "postgres://ds:ds@postgres:5432/datasources" : "postgres://ds:ds@127.0.0.1:15432/datasources");

const compose = (...args) =>
  spawnSync("docker", ["compose", "-p", project, "-f", composeFile, ...args], { cwd: root, encoding: "utf8", stdio: ["ignore", "pipe", "pipe"] });

const requireOk = (result, label) => {
  if ((result.status ?? 1) !== 0)
    throw new Error(`${label}: ${result.stderr || result.stdout || "failed"}`);
  return result;
};

const leftover = () => {
  const listed = spawnSync("docker", ["ps", "-aq", "--filter", `name=${project}`], { encoding: "utf8" });
  const ids = (listed.stdout || "").trim().split(/\s+/).filter(Boolean);
  if (ids.length) spawnSync("docker", ["rm", "-f", ...ids], { encoding: "utf8", stdio: "inherit" });
};

const down = () => {
  compose("down", "-v", "--remove-orphans");
  leftover();
};

const up = () => {
  requireOk(compose("up", "-d", "--wait", "mysql", "postgres"), "compose up");
  compose("exec", "-T", "mysql", "mysql", "-uroot", "-proot", "-e", "GRANT ALL PRIVILEGES ON *.* TO 'ds'@'%'; FLUSH PRIVILEGES;");
};

const isolate = (provider) => {
  if (provider === "sqlite") return { url: "sqlite:///:memory:", drop: () => {} };
  const name = `ds_${crypto.randomUUID().replaceAll("-", "").slice(0, 16)}`;
  if (provider === "mysql") {
    requireOk(
      compose("exec", "-T", "mysql", "mysql", "-uds", "-pds", "-e", `CREATE DATABASE \`${name}\``),
      "create mysql",
    );
    const url = new URL(mysqlUrl());
    url.pathname = `/${name}`;
    return {
      url: url.toString(),
      drop: () => compose("exec", "-T", "mysql", "mysql", "-uds", "-pds", "-e", `DROP DATABASE IF EXISTS \`${name}\``),
    };
  }
  requireOk(
    compose("exec", "-T", "postgres", "psql", "-U", "ds", "-d", "datasources", "-c", `CREATE SCHEMA ${name}`),
    "create postgres",
  );
  const base = postgresUrl();
  return {
    url: `${base}${base.includes("?") ? "&" : "?"}schema=${name}`,
    drop: () =>
      compose("exec", "-T", "postgres", "psql", "-U", "ds", "-d", "datasources", "-c", `DROP SCHEMA IF EXISTS ${name} CASCADE`),
  };
};

const spec = join(root, "samples/contacts-crud-enriched-medium-rest-shared/deterministic");

const generate = (kind, dialect, out) => {
  mkdirSync(out, { recursive: true });
  requireOk(
    spawnSync(
      process.execPath,
      [
        "--experimental-strip-types",
        join(root, "cli/src/main.ts"),
        "generate",
        kind,
        "--dialect",
        dialect,
        "--input",
        spec,
        "--output",
        out,
      ],
      { cwd: join(root, "cli"), encoding: "utf8" },
    ),
    `generate ${kind} ${dialect}`,
  );
  return out;
};

const generateSql = () => {
  const out = generate("datasource", "typescript", join(artifacts, "generated"));
  for (const provider of providers) {
    const sql = join(out, "sql", provider, "migrations/0001_initial_up.sql");
    if (!existsSync(sql)) throw new Error(`missing generated sql ${sql}`);
  }
  return (provider) => join(out, "sql", provider, "migrations/0001_initial_up.sql");
};

const generateCsharp = () => generate("all", "csharp", join(artifacts, "generated-csharp"));

const harness = (sample, dialect) => {
  if (dialect === "typescript")
    return {
      command: process.execPath,
      args: ["--import", "tsx", join(root, "samples/typescript", sample, "matrix.ts")],
      cwd: join(root, "samples/typescript", sample),
    };
  if (dialect === "rust")
    return {
      command: "cargo",
      args: ["run", "--quiet", "--manifest-path", join(root, "samples/rust", sample, "Cargo.toml"), "--bin", "matrix"],
      cwd: join(root, "samples/rust", sample),
    };
  return {
    command: "dotnet",
    args: ["run", "--project", join(root, "samples/csharp", sample, "matrix/Matrix.csproj"), "-p:CollectCoverage=false"],
    cwd: join(root, "samples/csharp", sample, "matrix"),
  };
};

const diffLines = (left, right) => {
  const n = Math.max(left.length, right.length);
  const out = [];
  for (let i = 0; i < n; i++) {
    if (left[i] === right[i]) out.push(` ${left[i] ?? ""}`);
    else {
      if (left[i] != null) out.push(`-${left[i]}`);
      if (right[i] != null) out.push(`+${right[i]}`);
    }
  }
  return out;
};

const escape = (text) =>
  String(text ?? "")
    .replaceAll("&", "&amp;")
    .replaceAll("<", "&lt;")
    .replaceAll(">", "&gt;")
    .replaceAll('"', "&quot;");

const readText = (path) => (existsSync(path) ? readFileSync(path, "utf8") : "");

const panel = (title, text) =>
  text.trim().length === 0 ? "" : `<h4>${escape(title)}</h4><pre>${escape(text)}</pre>`;

const cellId = (sample, dialect, provider) => `${sample}--${dialect}--${provider}`;

const firstHunk = (text, limit = 8) => {
  const lines = text.split(/\r?\n/).filter((line) => line.startsWith("+") || line.startsWith("-"));
  return lines.slice(0, limit).join("\n");
};

const firstError = (text) =>
  text
    .split(/\r?\n/)
    .map((line) => line.trim())
    .find((line) => line.length > 0 && !line.startsWith("Determining") && !line.startsWith("Restored")) ?? "";

const failuresOf = (cells) => {
  const crashed = cells.filter((cell) => !cell.ok);
  const diffs = cells.filter((cell) => cell.ok && cell.match === false);
  return { crashed, diffs };
};

const report = (cells, durationMs, cleanup) => {
  mkdirSync(join(artifacts, "normalized"), { recursive: true });
  mkdirSync(join(artifacts, "diffs"), { recursive: true });
  const { crashed, diffs } = failuresOf(cells);
  const summaryLines = [
    `e2e-matrix: ${cells.length} cells · ${crashed.length} failed to run · ${diffs.length} log diffs vs typescript/sqlite`,
    "",
    "Failed to run",
    ...(crashed.length
      ? crashed.map((cell) => {
          const err = firstError(readText(join(artifacts, "logs", cell.sample, cell.dialect, `${cell.provider}.stderr.txt`)));
          return `- ${cell.sample} ${cell.dialect}/${cell.provider}${err ? `: ${err}` : ""}`;
        })
      : ["- none"]),
    "",
    "Log diffs",
    ...(diffs.length
      ? diffs.map((cell) => {
          const hunk = firstHunk(readText(join(artifacts, "diffs", cell.sample, `${cell.dialect}-${cell.provider}.txt`)));
          return `- ${cell.sample} ${cell.dialect}/${cell.provider}${hunk ? `\n${hunk}` : ""}`;
        })
      : ["- none"]),
    "",
  ];
  writeFileSync(join(artifacts, "summary.txt"), `${summaryLines.join("\n")}\n`);
  const bySample = {};
  for (const cell of cells) (bySample[cell.sample] ??= []).push(cell);
  const summaryHtml = `<section id="failures"><h2>Failures</h2>
<p>${crashed.length} failed to run · ${diffs.length} log diffs · open <code>artifacts/e2e-matrix/summary.txt</code> for the same list</p>
<h3>Failed to run</h3><ul>${
    crashed.length
      ? crashed
          .map((cell) => {
            const err = firstError(readText(join(artifacts, "logs", cell.sample, cell.dialect, `${cell.provider}.stderr.txt`)));
            return `<li><a href="#${cellId(cell.sample, cell.dialect, cell.provider)}">${escape(cell.sample)} ${escape(cell.dialect)}/${escape(cell.provider)}</a>${err ? ` — ${escape(err)}` : ""}</li>`;
          })
          .join("")
      : "<li>none</li>"
  }</ul>
<h3>Log diffs vs typescript/sqlite</h3><ul>${
    diffs.length
      ? diffs
          .map((cell) => {
            const hunk = firstHunk(readText(join(artifacts, "diffs", cell.sample, `${cell.dialect}-${cell.provider}.txt`)));
            return `<li><a href="#${cellId(cell.sample, cell.dialect, cell.provider)}">${escape(cell.sample)} ${escape(cell.dialect)}/${escape(cell.provider)}</a>${hunk ? `<pre>${escape(hunk)}</pre>` : ""}</li>`;
          })
          .join("")
      : "<li>none</li>"
  }</ul></section>`;
  const sections = samples.map(({ name }) => {
    const rows = dialects
      .map((dialect) => {
        const tds = providers
          .map((provider) => {
            const cell = (bySample[name] ?? []).find((item) => item.dialect === dialect && item.provider === provider);
            const cls = !cell?.ok ? "fail" : cell.match === false ? "mismatch" : "ok";
            const compare = cell?.match === true ? "match" : cell?.match === false ? "diff" : "";
            const id = cellId(name, dialect, provider);
            return `<td class="${cls}"><a href="#${id}">${provider}</a><div>${cell?.ok ? "ran" : "failed"} ${compare}</div></td>`;
          })
          .join("");
        return `<tr><th>${dialect}</th>${tds}</tr>`;
      })
      .join("");
    const mismatches = (bySample[name] ?? [])
      .filter((cell) => cell.match === false)
      .map((cell) => `<li><a href="#${cellId(name, cell.dialect, cell.provider)}">${cell.dialect}/${cell.provider}</a></li>`)
      .join("");
    const details = dialects
      .flatMap((dialect) =>
        providers.map((provider) => {
          const id = cellId(name, dialect, provider);
          const raw = readText(join(artifacts, "logs", name, dialect, `${provider}.log`));
          const err = readText(join(artifacts, "logs", name, dialect, `${provider}.stderr.txt`));
          const norm = readText(join(artifacts, "normalized", name, `${dialect}-${provider}.txt`));
          const diff = readText(join(artifacts, "diffs", name, `${dialect}-${provider}.txt`));
          return `<article id="${id}"><h3>${escape(name)} ${escape(dialect)} ${escape(provider)} <a href="#top">top</a></h3>${panel("log", raw)}${panel("normalized", norm)}${panel("diff vs typescript/sqlite", diff)}${panel("stderr", err) || (raw.trim() || err.trim() ? "" : "<p>no log</p>")}</article>`;
        }),
      )
      .join("");
    return `<h2 id="${name}">${name}</h2><table><thead><tr><th></th>${providers.map((p) => `<th>${p}</th>`).join("")}</tr></thead><tbody>${rows}</tbody></table><h3>Mismatches vs typescript/sqlite</h3><ul>${mismatches || "<li>none</li>"}</ul>${details}`;
  });
  const html = `<!DOCTYPE html><html><head><meta charset="utf-8"><title>e2e-matrix</title>
<style>
body{font-family:system-ui,sans-serif;margin:2rem;background:#111;color:#eee}
table{border-collapse:collapse;margin:1rem 0}
th,td{border:1px solid #444;padding:.5rem .75rem}
a{color:#8cf}
.ok{background:#143}
.fail{background:#411}
.mismatch{background:#542}
footer{margin-top:2rem;color:#aaa}
article{margin:2rem 0;padding-top:1rem;border-top:1px solid #444}
pre{white-space:pre-wrap;background:#1a1a1a;padding:1rem;overflow:auto}
h3{scroll-margin-top:1rem}
</style></head><body>
<h1 id="top">e2e-matrix</h1>
${summaryHtml}
${sections.join("\n")}
<footer>duration ${Math.round(durationMs / 1000)}s · cleanup ${cleanup} · reference typescript/sqlite</footer>
</body></html>`;
  writeFileSync(join(artifacts, "index.html"), html);
};

const scanCells = () =>
  samples.flatMap(({ name }) =>
    dialects.flatMap((dialect) =>
      providers.map((provider) => {
        const log = join(artifacts, "logs", name, dialect, `${provider}.log`);
        const stderr = join(artifacts, "logs", name, dialect, `${provider}.stderr.txt`);
        const diff = join(artifacts, "diffs", name, `${dialect}-${provider}.txt`);
        const norm = join(artifacts, "normalized", name, `${dialect}-${provider}.txt`);
        return {
          sample: name,
          dialect,
          provider,
          ok: existsSync(log) && !existsSync(stderr),
          match: existsSync(diff) ? false : existsSync(norm) ? true : undefined,
          log,
        };
      }),
    ),
  );

if (process.argv.includes("--report-only")) {
  report(scanCells(), 0, "rebuilt");
  process.stderr.write(`report ${join(artifacts, "index.html")}\n`);
  process.stderr.write(`summary ${join(artifacts, "summary.txt")}\n`);
  process.exit(0);
}

const runCell = (sample, dialect, provider, sql, csharpOut) => {
  const log = join(artifacts, "logs", sample.name, dialect, `${provider}.log`);
  mkdirSync(dirname(log), { recursive: true });
  const db = isolate(provider);
  const started = Date.now();
  try {
    const { command, args, cwd } = harness(sample.name, dialect);
    if (csharpOut) args.push(`-p:E2E_MATRIX_GENERATED=${csharpOut}`);
    const env = {
      ...process.env,
      PROVIDER: provider,
      DATASOURCE_URL: db.url,
      E2E_MATRIX_LOG: log,
      MSBUILDDISABLENODEREUSE: "1",
    };
    if (sql) env.E2E_MATRIX_SQL = sql;
    if (csharpOut) env.E2E_MATRIX_GENERATED = csharpOut;
    const result = spawnSync(command, args, { cwd, env, encoding: "utf8", maxBuffer: 20 * 1024 * 1024 });
    const ok = (result.status ?? 1) === 0;
    const stderr = join(artifacts, "logs", sample.name, dialect, `${provider}.stderr.txt`);
    if (ok) rmSync(stderr, { force: true });
    else writeFileSync(stderr, `${result.stdout}\n${result.stderr}`);
    return { sample: sample.name, dialect, provider, ok, ms: Date.now() - started, log };
  } finally {
    db.drop();
  }
};

const compare = (cells) => {
  for (const { name } of samples) {
    const reference = cells.find((cell) => cell.sample === name && cell.dialect === "typescript" && cell.provider === "sqlite");
    const logged = (cell) => cell && existsSync(cell.log) && readFileSync(cell.log, "utf8").trim().length > 0;
    const expected = logged(reference) ? dialectTranscript(readFileSync(reference.log, "utf8").split(/\r?\n/)) : null;
    mkdirSync(join(artifacts, "normalized", name), { recursive: true });
    if (expected) writeFileSync(join(artifacts, "normalized", name, "typescript-sqlite.txt"), `${expected.join("\n")}\n`);
    for (const cell of cells.filter((item) => item.sample === name)) {
      if (!logged(cell) || !expected) {
        cell.match = undefined;
        continue;
      }
      const actual = dialectTranscript(readFileSync(cell.log, "utf8").split(/\r?\n/));
      mkdirSync(join(artifacts, "normalized", name), { recursive: true });
      writeFileSync(join(artifacts, "normalized", name, `${cell.dialect}-${cell.provider}.txt`), `${actual.join("\n")}\n`);
      cell.match = actual.join("\n") === expected.join("\n");
      if (!cell.match) {
        mkdirSync(join(artifacts, "diffs", name), { recursive: true });
        writeFileSync(
          join(artifacts, "diffs", name, `${cell.dialect}-${cell.provider}.txt`),
          `${diffLines(expected, actual).join("\n")}\n`,
        );
      }
    }
  }
};

const started = Date.now();
mkdirSync(artifacts, { recursive: true });
let cleanup = "pending";
const cells = [];
try {
  if (!inDocker) {
    down();
    up();
  }
  const sqlFor = samples.some((sample) => sample.sql) ? generateSql() : undefined;
  const csharpOut = samples.some((sample) => sample.sql) ? generateCsharp() : undefined;
  for (const sample of samples) {
    for (const dialect of dialects) {
      for (const provider of providers) {
        process.stderr.write(`${sample.name} ${dialect} ${provider}\n`);
        try {
          cells.push(runCell(sample, dialect, provider, sample.sql ? sqlFor(provider) : undefined, sample.sql && dialect === "csharp" ? csharpOut : undefined));
        } catch (error) {
          const log = join(artifacts, "logs", sample.name, dialect, `${provider}.log`);
          mkdirSync(dirname(log), { recursive: true });
          writeFileSync(join(artifacts, "logs", sample.name, dialect, `${provider}.stderr.txt`), `${error instanceof Error ? error.stack : error}\n`);
          cells.push({ sample: sample.name, dialect, provider, ok: false, ms: 0, log });
        }
      }
    }
  }
  compare(cells);
} finally {
  try {
    if (!inDocker) down();
    cleanup = "ok";
  } catch (error) {
    cleanup = `failed: ${error instanceof Error ? error.message : error}`;
  }
}
report(cells, Date.now() - started, cleanup);
const failed = cells.some((cell) => !cell.ok || cell.match === false);
process.stderr.write(`report ${join(artifacts, "index.html")}\n`);
process.stderr.write(`summary ${join(artifacts, "summary.txt")}\n`);
process.exit(failed ? 1 : 0);
