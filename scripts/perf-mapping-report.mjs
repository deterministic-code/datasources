import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const jsonl = process.argv[2];
const outDir = process.argv[3];
if (!jsonl || !outDir) throw new Error("usage: perf-mapping-report.mjs <jsonl> <out-dir>");

const records = readFileSync(jsonl, "utf8")
  .split(/\r?\n/)
  .map((line) => line.trim())
  .filter((line) => line.startsWith("{"))
  .map((line) => JSON.parse(line));
const containers = records.filter((row) => row.phase === "container");
const results = records.filter((row) => row.sample === "mapping_comparison");
const languages = [...new Set([...containers, ...results].map((row) => row.language).filter(Boolean))];
const providers = [...new Set(results.map((row) => row.provider).filter(Boolean))];
const tests = ["defaultRead", "dictionaryRead", "propertyMappedRead"];
const round = (value) => Math.round(Number(value));
const found = (language, provider) =>
  results.find((row) => row.language === language && row.provider === provider);
const escape = (text) =>
  String(text).replaceAll("&", "&amp;").replaceAll("<", "&lt;").replaceAll(">", "&gt;");
const metric = (value, unit) =>
  value == null || Number.isNaN(Number(value))
    ? "<td></td>"
    : `<td class="ok">${round(value)}<sub>${unit}</sub></td>`;

const containerRows = languages
  .map((language) => {
    const row = containers.find((item) => item.language === language);
    return `<tr><th>${escape(language)}</th>${metric(row?.containerBuildMs, "ms")}</tr>`;
  })
  .join("");

const resultRows = languages
  .flatMap((language) =>
    providers.flatMap((provider) =>
      tests.map((test) => {
        const row = found(language, provider);
        const cell = row?.tests?.[test];
        if (!cell)
          return `<tr><th>${escape(language)}</th><th>${escape(provider)}</th><th>${escape(test)}</th><td></td><td></td><td></td><td></td><td></td></tr>`;
        return `<tr><th>${escape(language)}</th><th>${escape(provider)}</th><th>${escape(test)}</th>${metric(cell.seedMs, "ms")}${metric(cell.seedCpuMs, "ms")}${metric(cell.readMs, "ms")}${metric(cell.readCpuMs, "ms")}${metric(cell.readPerSec, "parents p/s")}</tr>`;
      }),
    ),
  )
  .join("");

mkdirSync(outDir, { recursive: true });
writeFileSync(
  join(outDir, "mapping-comparison.json"),
  `${JSON.stringify({ containers, results }, null, 2)}\n`,
);
writeFileSync(
  join(outDir, "index.html"),
  `<!DOCTYPE html><html><head><meta charset="utf-8"><title>mapping comparison</title>
<style>
body{font-family:system-ui,sans-serif;margin:2rem;background:#111;color:#eee}
table{border-collapse:collapse;margin:1rem 0}
th,td{border:1px solid #444;padding:.65rem 1rem;vertical-align:top;white-space:nowrap}
.ok{background:#143;font-variant-numeric:tabular-nums;line-height:1.55}
sub{color:#aaa;font-size:.65em}
</style></head><body>
<h1>mapping comparison</h1>
<p>each test seeds, reads, then deletes. seed and read are wall-clock; seed cpu and read cpu are process CPU. container build is docker compose build.</p>
<h2>container</h2>
<table><thead><tr><th>language</th><th>build</th></tr></thead><tbody>${containerRows}</tbody></table>
<h2>tests</h2>
<table><thead><tr><th>language</th><th>provider</th><th>test</th><th>seed</th><th>seed cpu</th><th>read</th><th>read cpu</th><th>read rate</th></tr></thead><tbody>${resultRows}</tbody></table>
</body></html>
`,
);
process.stderr.write(`report ${join(outDir, "index.html")}\n`);
