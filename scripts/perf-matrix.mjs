import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { writePerfData } from "./perf-data.mjs";

const root = dirname(fileURLToPath(new URL(".", import.meta.url)));
const artifacts = join(root, "scripts/artifacts/perf");
const composeFile = join(root, "docker/perf/compose.yml");
const project = "datasources-perf";
const list = (name, fallback) =>
  (process.env[name] ?? fallback).split(",").map((part) => part.trim()).filter(Boolean);
const languages = list("PERF_LANGUAGES", "typescript,rust,csharp");
const providers = list("PERF_PROVIDERS", "sqlite,mysql,postgres");
const samples = list("PERF_SAMPLES", "user-roles-simple-cli,contacts-crud-enriched-medium-rest");
const rows = Number(process.env.PERF_ROWS ?? 1000);
const chunk = Number(process.env.PERF_CHUNK ?? 500);
const related = Number(process.env.PERF_RELATED ?? 10);
const strategies = list("PERF_STRATEGIES", "single,batch,procedure");

const mysqlUrl = () => process.env.MYSQL_URL || "mysql://ds:ds@127.0.0.1:23306/datasources";
const postgresUrl = () => process.env.POSTGRES_URL || "postgres://ds:ds@127.0.0.1:25432/datasources";

const compose = (...args) =>
  spawnSync("docker", ["compose", "-p", project, "-f", composeFile, ...args], {
    cwd: root,
    encoding: "utf8",
    stdio: ["ignore", "pipe", "pipe"],
  });

const leftover = () => {
  const listed = spawnSync("docker", ["ps", "-aq", "--filter", `name=${project}`], { encoding: "utf8" });
  const ids = (listed.stdout || "").trim().split(/\s+/).filter(Boolean);
  if (ids.length) spawnSync("docker", ["rm", "-f", ...ids], { encoding: "utf8" });
  const volumes = spawnSync("docker", ["volume", "ls", "-q", "--filter", `name=${project}`], { encoding: "utf8" });
  const names = (volumes.stdout || "").trim().split(/\s+/).filter(Boolean);
  if (names.length) spawnSync("docker", ["volume", "rm", "-f", ...names], { encoding: "utf8" });
};

const down = () => {
  compose("down", "-v", "--remove-orphans");
  leftover();
};

const reading = process.argv.includes("--reads");
const contactsSample = "contacts-crud-enriched-medium-rest";
const keepStore = (sample, strategy) => sample === contactsSample && strategy === "batch";
const storesPath = join(artifacts, "stores.json");

const stop = (code = 0) => {
  if (!reading) down();
  process.exit(code);
};

process.on("SIGINT", () => stop(130));
process.on("SIGTERM", () => stop(143));

const up = (service) => {
  const result = compose("up", "-d", "--wait", service);
  if ((result.status ?? 1) !== 0) throw new Error(`compose up ${service}: ${result.stderr || result.stdout}`);
  if (service === "mysql")
    compose("exec", "-T", "mysql", "mysql", "-uroot", "-proot", "-e", "GRANT ALL PRIVILEGES ON *.* TO 'ds'@'%'; FLUSH PRIVILEGES;");
};

const isolate = (provider, persist, id) => {
  if (provider === "sqlite") {
    if (persist) {
      const dir = join(artifacts, "stores");
      mkdirSync(dir, { recursive: true });
      const file = join(dir, `${id}.db`);
      rmSync(file, { force: true });
      return { url: `sqlite:///${file}`, drop: () => {} };
    }
    const dir = mkdtempSync(join(tmpdir(), "ds-perf-"));
    return { url: `sqlite:///${join(dir, "perf.db")}`, drop: () => rmSync(dir, { recursive: true, force: true }) };
  }
  const name = persist ? `ds_${id.replaceAll(/[^a-z0-9]/gi, "").slice(0, 32)}` : `ds_${crypto.randomUUID().replaceAll("-", "").slice(0, 16)}`;
  if (provider === "mysql") {
    compose("exec", "-T", "mysql", "mysql", "-uds", "-pds", "-e", `CREATE DATABASE \`${name}\``);
    const url = new URL(mysqlUrl());
    url.pathname = `/${name}`;
    return {
      url: url.toString(),
      drop: persist
        ? () => {}
        : () => compose("exec", "-T", "mysql", "mysql", "-uds", "-pds", "-e", `DROP DATABASE IF EXISTS \`${name}\``),
    };
  }
  compose("exec", "-T", "postgres", "psql", "-U", "ds", "-d", "datasources", "-c", `CREATE SCHEMA ${name}`);
  const base = postgresUrl();
  return {
    url: `${base}${base.includes("?") ? "&" : "?"}schema=${name}`,
    drop: persist
      ? () => {}
      : () =>
          compose("exec", "-T", "postgres", "psql", "-U", "ds", "-d", "datasources", "-c", `DROP SCHEMA IF EXISTS ${name} CASCADE`),
  };
};

const rustCwd = (sample) => join(root, "samples/rust", sample);
const csharpCwd = (sample) => join(root, "samples/csharp", sample, "perf");
const tsCwd = (sample) => join(root, "samples/typescript", sample);

const rustTarget = (sample) => join(rustCwd(sample), "target");
const hasRustPerf = (sample) => existsSync(join(rustCwd(sample), "src/bin/perf.rs"));

const compile = () => {
  for (const sample of samples) {
    if (hasRustPerf(sample)) {
      const rust = spawnSync("cargo", ["build", "--release", "--quiet", "--bin", "perf"], {
        cwd: rustCwd(sample),
        encoding: "utf8",
        env: { ...process.env, CARGO_TARGET_DIR: rustTarget(sample) },
      });
      if ((rust.status ?? 1) !== 0) throw new Error(`cargo build ${sample}: ${rust.stderr || rust.stdout}`);
    }
    const csharp = spawnSync(
      "dotnet",
      ["build", "-c", "Release", "-p:CollectCoverage=false", "--nologo", "-v", "q"],
      { cwd: csharpCwd(sample), encoding: "utf8" },
    );
    if ((csharp.status ?? 1) !== 0) throw new Error(`dotnet build ${sample}: ${csharp.stderr || csharp.stdout}`);
  }
};

const run = (sample, language, provider, strategy, url) => {
  const { PERF_READS: _omitReads, ...parent } = process.env;
  const env = {
    ...parent,
    PROVIDER: provider,
    DATASOURCE_URL: url,
    PERF_STRATEGY: strategy,
    PERF_ROWS: String(rows),
    PERF_CHUNK: String(chunk),
    PERF_RELATED: String(related),
    PERF_DATA: data[sample],
    ...(language === "rust" ? { CARGO_TARGET_DIR: rustTarget(sample) } : {}),
    PERF_PHASE: reading ? "read" : "write",
    ...(reading
      ? {
          PERF_READ_ITEMS: process.env.PERF_READ_ITEMS ?? "100",
          PERF_READ_ITERS: process.env.PERF_READ_ITERS ?? "5",
        }
      : {}),
  };
  const cwd =
    language === "typescript" ? tsCwd(sample) : language === "rust" ? rustCwd(sample) : csharpCwd(sample);
  const command =
    language === "typescript"
      ? [process.execPath, "--import", "tsx", "perf.ts"]
      : language === "rust"
        ? [join(rustTarget(sample), "release", "perf")]
        : ["dotnet", "run", "-c", "Release", "--no-build", "--project", "Perf.csproj", "-p:CollectCoverage=false"];
  const result = spawnSync(command[0], command.slice(1), { cwd, encoding: "utf8", env });
  const line = (result.stdout || "")
    .trim()
    .split(/\r?\n/)
    .filter((text) => text.startsWith("{"))
    .at(-1);
  if ((result.status ?? 1) !== 0 || !line)
    return {
      sample,
      language,
      provider,
      strategy,
      rows,
      ok: false,
      error: usefulError(result.stderr || result.stdout || "failed"),
    };
  return { ...JSON.parse(line), ok: true };
};

const writeAtomic = (name, text) => {
  const dest = join(artifacts, name);
  const temp = `${dest}.tmp`;
  writeFileSync(temp, text);
  renameSync(temp, dest);
};

const mergeCells = (fresh) => {
  const path = join(artifacts, "results.json");
  const sliced = ["PERF_SAMPLES", "PERF_LANGUAGES", "PERF_PROVIDERS", "PERF_STRATEGIES"].some(
    (name) => process.env[name],
  );
  if ((!reading && !sliced) || !existsSync(path) || process.env.PERF_REPLACE === "1") return fresh;
  const prior = JSON.parse(readFileSync(path, "utf8"));
  const key = (cell) => [cell.sample, cell.language, cell.provider, cell.strategy].join("\t");
  const by = new Map(prior.map((cell) => [key(cell), cell]));
  for (const cell of fresh) {
    const priorCell = by.get(key(cell));
    by.set(
      key(cell),
      reading && priorCell?.ok
        ? { ...priorCell, reads: cell.reads, ok: cell.ok, error: cell.error }
        : cell,
    );
  }
  return [...by.values()];
};

const report = (cells) => {
  cells = mergeCells(cells);
  mkdirSync(artifacts, { recursive: true });
  writeAtomic("results.json", `${JSON.stringify(cells, null, 2)}\n`);
  const cell = (sample, language, provider, strategy) =>
    cells.find(
      (item) =>
        item.sample === sample && item.language === language && item.provider === provider && item.strategy === strategy,
    );
  const sections = samples
    .map((sample) => {
      const rowsHtml = languages
        .flatMap((language) =>
          providers.map((provider) => {
            const tds = strategies
              .map((strategy) => {
                const found =
                  cell(sample, language, provider, strategy) ??
                  (provider === "sqlite" && strategy === "procedure"
                    ? cell(sample, language, provider, "batch")
                    : undefined);
                if (!found) return "<td></td>";
                if (!found.ok) return `<td class="fail">${escape(found.error || "failed")}</td>`;
                const used = provider === "sqlite" && strategy === "procedure" ? " · batch used" : "";
                const elapsed = found.elapsed ?? found.createMs;
                const perSec = found.rowsPerSec ?? found.createPerSec;
                return `<td class="ok">${found.rows}<sub>rows</sub><br>${Math.round(elapsed)}<sub>elapsed ms.</sub><br>${Math.round(perSec)}<sub>row p/s</sub>${used ? `<div>${used}</div>` : ""}</td>`;
              })
              .join("");
            return `<tr><th>${language}</th><th>${provider}</th>${tds}</tr>`;
          }),
        )
        .join("");
      const readTables =
        sample === "contacts-crud-enriched-medium-rest" && cells.some((item) => item.reads)
          ? ["ids", "where", "firstName"]
              .map((shape) => {
                const body = languages
                  .flatMap((language) =>
                    providers.map((provider) => {
                      const found = cell(sample, language, provider, "batch");
                      const tds = ["default", "hashset", "flat"]
                        .map((mode) => {
                          if (!found) return "<td></td>";
                          if (!found.ok) return `<td class="fail">${escape(found.error || "failed")}</td>`;
                          const stat = found.reads?.[mode]?.[shape];
                          if (!stat) return "<td></td>";
                          return `<td class="ok">${Math.round(stat.ms)}<sub>ms</sub><br>${Math.round(stat.perSec)}<sub>parents p/s</sub></td>`;
                        })
                        .join("");
                      return `<tr><th>${language}</th><th>${provider}</th>${tds}</tr>`;
                    }),
                  )
                  .join("");
                return `<h3>${shape}</h3>
<p>median ms and parents/s after the batch write. default, hashset, flat-to-nested.</p>
<table><thead><tr><th>language</th><th>provider</th><th>default</th><th>hashset</th><th>flat</th></tr></thead><tbody>${body}</tbody></table>`;
              })
              .join("\n")
          : "";
      return `<h2 id="${sample}">${sample}</h2>
<p>rows, elapsed ms, rows/s. insert only.</p>
<table><thead><tr><th>language</th><th>provider</th><th>single</th><th>batch</th><th>stored procedure</th></tr></thead><tbody>${rowsHtml}</tbody></table>
${readTables}`;
    })
    .join("\n");
  writeAtomic(
    "index.html",
    `<!DOCTYPE html><html><head><meta charset="utf-8"><title>bulk performance</title>
<style>
body{font-family:system-ui,sans-serif;margin:2rem;background:#111;color:#eee}
table{border-collapse:collapse;margin:1rem 0}
th,td{border:1px solid #444;padding:.65rem 1rem;vertical-align:top;white-space:nowrap}
th:nth-child(n+3),td.ok,td.fail{min-width:11rem}
.ok{background:#143;font-variant-numeric:tabular-nums;line-height:1.55}
.fail{background:#411}
.skip{background:#333;color:#aaa}
div{color:#aaa;font-size:.85rem}
sub{color:#aaa;font-size:.65em}
</style></head><body>
<h1>BulkCRUDRepository performance</h1>
${sections}
</body></html>
`,
  );
};

const usefulError = (text) => {
  const lines = String(text)
    .trim()
    .split(/\r?\n/)
    .map((line) => line.trim())
    .filter(Boolean)
    .filter(
      (line) =>
        !/^Node\.js /.test(line) &&
        !/^npm /.test(line) &&
        !/^at /.test(line) &&
        line !== "---" &&
        line !== "{" &&
        line !== "}",
    );
  const named = lines.findLast(
    (line) =>
      /sqlMessage|Server error|Unhandled exception|ERROR [0-9A-Z]+ |Incorrect |Illegal mix|relation |syntax/i.test(
        line,
      ) && line !== "db error",
  );
  if (named) return named.slice(0, 400);
  return (lines.join(" ") || String(text).trim()).slice(-400) || "failed";
};

const escape = (text) =>
  String(text).replaceAll("&", "&amp;").replaceAll("<", "&lt;").replaceAll(">", "&gt;");

if (process.argv.includes("--report-only")) {
  report(JSON.parse(readFileSync(join(artifacts, "results.json"), "utf8")));
  process.stderr.write(`report ${join(artifacts, "index.html")}\n`);
  process.exit(0);
}

mkdirSync(artifacts, { recursive: true });
const data = writePerfData(join(artifacts, "data"), rows, related);
const engines = () => [...new Set(providers.filter((provider) => provider !== "sqlite"))];
const cells = [];
const saveStores = (stores) => writeAtomic("stores.json", `${JSON.stringify(stores, null, 2)}\n`);

if (reading) {
  if (!existsSync(storesPath)) throw new Error("run npm run test:perf first");
  const stores = JSON.parse(readFileSync(storesPath, "utf8"));
  if (!stores.cells?.length) throw new Error("run npm run test:perf first");
  compile();
  try {
    for (const provider of [...new Set(stores.cells.map((cell) => cell.provider))])
      if (provider !== "sqlite") up(provider);
    for (const store of stores.cells) {
      if (languages.length && !languages.includes(store.language)) continue;
      if (store.language === "rust" && !hasRustPerf(store.sample)) continue;
      try {
        cells.push(run(store.sample, store.language, store.provider, store.strategy, store.url));
      } catch (error) {
        cells.push({
          ...store,
          rows: stores.rows ?? rows,
          ok: false,
          error: String(error.message || error),
        });
      }
      report(cells);
    }
  } catch (error) {
    process.stderr.write(`${error.message || error}\n`);
    process.exit(1);
  }
  report(cells);
  process.stderr.write(`report ${join(artifacts, "index.html")}\n`);
  if (cells.some((cell) => !cell.ok)) process.exit(1);
  process.exit(0);
}

down();
compile();
const stores = { rows, related, cells: [] };
try {
  for (const provider of engines()) up(provider);
  for (const provider of providers)
    for (const sample of samples)
      for (const language of languages)
        for (const strategy of strategies) {
          if (language === "rust" && !hasRustPerf(sample)) continue;
          const persist = keepStore(sample, strategy);
          const id = `${sample}_${language}_${provider}_${strategy}`.replaceAll(/[^a-z0-9_]/gi, "_");
          const isolated = isolate(provider, persist, id);
          try {
            const cell = run(sample, language, provider, strategy, isolated.url);
            cells.push(cell);
            if (persist && cell.ok) stores.cells.push({ sample, language, provider, strategy, url: isolated.url });
          } catch (error) {
            cells.push({
              sample,
              language,
              provider,
              strategy,
              rows,
              ok: false,
              error: String(error.message || error),
            });
          } finally {
            isolated.drop();
          }
          saveStores(stores);
          report(cells);
        }
} finally {
  saveStores(stores);
}
report(cells);
process.stderr.write(`report ${join(artifacts, "index.html")}\n`);
if (cells.some((cell) => !cell.ok)) process.exit(1);
