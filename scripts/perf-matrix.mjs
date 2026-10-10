import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const root = dirname(fileURLToPath(new URL(".", import.meta.url)));
const artifacts = join(root, "artifacts/perf");
const composeFile = join(root, "docker/perf/compose.yml");
const project = "datasources-perf";
const languages = ["typescript", "rust", "csharp"];
const providers = ["sqlite", "mysql", "postgres"];
const samples = ["user-roles-simple-cli", "contacts-crud-enriched-medium-rest"];
const rows = Number(process.env.PERF_ROWS ?? 1000);
const chunk = Number(process.env.PERF_CHUNK ?? 500);
const strategiesOf = (provider) =>
  provider === "sqlite" ? ["single", "batch"] : ["single", "batch", "procedure"];

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
};

const down = () => {
  compose("down", "-v", "--remove-orphans");
  leftover();
};

const up = (service) => {
  const result = compose("up", "-d", "--wait", service);
  if ((result.status ?? 1) !== 0) throw new Error(`compose up ${service}: ${result.stderr || result.stdout}`);
  if (service === "mysql")
    compose("exec", "-T", "mysql", "mysql", "-uroot", "-proot", "-e", "GRANT ALL PRIVILEGES ON *.* TO 'ds'@'%'; FLUSH PRIVILEGES;");
};

const isolate = (provider) => {
  if (provider === "sqlite") {
    const dir = mkdtempSync(join(tmpdir(), "ds-perf-"));
    return { url: `sqlite:///${join(dir, "perf.db")}`, drop: () => rmSync(dir, { recursive: true, force: true }) };
  }
  const name = `ds_${crypto.randomUUID().replaceAll("-", "").slice(0, 16)}`;
  if (provider === "mysql") {
    compose("exec", "-T", "mysql", "mysql", "-uds", "-pds", "-e", `CREATE DATABASE \`${name}\``);
    const url = new URL(mysqlUrl());
    url.pathname = `/${name}`;
    return {
      url: url.toString(),
      drop: () => compose("exec", "-T", "mysql", "mysql", "-uds", "-pds", "-e", `DROP DATABASE IF EXISTS \`${name}\``),
    };
  }
  compose("exec", "-T", "postgres", "psql", "-U", "ds", "-d", "datasources", "-c", `CREATE SCHEMA ${name}`);
  const base = postgresUrl();
  return {
    url: `${base}${base.includes("?") ? "&" : "?"}schema=${name}`,
    drop: () =>
      compose("exec", "-T", "postgres", "psql", "-U", "ds", "-d", "datasources", "-c", `DROP SCHEMA IF EXISTS ${name} CASCADE`),
  };
};

const run = (sample, language, provider, strategy, url) => {
  const env = {
    ...process.env,
    PROVIDER: provider,
    DATASOURCE_URL: url,
    PERF_STRATEGY: strategy,
    PERF_ROWS: String(rows),
    PERF_CHUNK: String(chunk),
  };
  const cwd =
    language === "typescript"
      ? join(root, "samples/typescript", sample)
      : language === "rust"
        ? join(root, "samples/rust", sample)
        : join(root, "samples/csharp", sample, "perf");
  const command =
    language === "typescript"
      ? [process.execPath, "--import", "tsx", "perf.ts"]
      : language === "rust"
        ? ["cargo", "run", "--quiet", "--bin", "perf"]
        : ["dotnet", "run", "--project", "Perf.csproj", "-p:CollectCoverage=false"];
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

const report = (cells) => {
  mkdirSync(artifacts, { recursive: true });
  writeFileSync(join(artifacts, "results.json"), `${JSON.stringify(cells, null, 2)}\n`);
  const cell = (sample, language, provider, strategy) =>
    cells.find(
      (item) =>
        item.sample === sample && item.language === language && item.provider === provider && item.strategy === strategy,
    );
  const sections = samples
    .map((sample) => {
      const strategies = ["single", "batch", "procedure"];
      const rowsHtml = languages
        .flatMap((language) =>
          providers.map((provider) => {
            const tds = strategies
              .map((strategy) => {
                if (provider === "sqlite" && strategy === "procedure")
                  return "<td class=\"skip\">batch used</td>";
                const found = cell(sample, language, provider, strategy);
                if (!found) return "<td></td>";
                if (!found.ok) return `<td class="fail">${escape(found.error || "failed")}</td>`;
                return `<td class="ok">${Math.round(found.createMs)} / ${Math.round(found.updateMs)}<div>${Math.round(found.createPerSec)} / ${Math.round(found.updatePerSec)} rows/s</div></td>`;
              })
              .join("");
            return `<tr><th>${language}</th><th>${provider}</th>${tds}</tr>`;
          }),
        )
        .join("");
      return `<h2 id="${sample}">${sample}</h2>
<p>create ms / update ms, then rows/s. ${rows} rows.</p>
<table><thead><tr><th>language</th><th>provider</th><th>single</th><th>batch</th><th>stored procedure</th></tr></thead><tbody>${rowsHtml}</tbody></table>`;
    })
    .join("\n");
  writeFileSync(
    join(artifacts, "index.html"),
    `<!DOCTYPE html><html><head><meta charset="utf-8"><title>bulk performance</title>
<style>
body{font-family:system-ui,sans-serif;margin:2rem;background:#111;color:#eee}
table{border-collapse:collapse;margin:1rem 0}
th,td{border:1px solid #444;padding:.5rem .75rem;vertical-align:top}
.ok{background:#143}
.fail{background:#411}
.skip{background:#333;color:#aaa}
div{color:#aaa;font-size:.85rem}
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
const cells = [];
try {
  for (const provider of providers) {
    if (provider !== "sqlite") {
      down();
      up(provider);
    }
    for (const sample of samples)
      for (const language of languages)
        for (const strategy of strategiesOf(provider)) {
          const isolated = isolate(provider);
          try {
            cells.push(run(sample, language, provider, strategy, isolated.url));
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
        }
    if (provider !== "sqlite") down();
  }
} finally {
  down();
}
report(cells);
process.stderr.write(`report ${join(artifacts, "index.html")}\n`);
if (cells.some((cell) => !cell.ok)) process.exit(1);
