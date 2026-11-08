#!/usr/bin/env node
import { readFileSync } from "node:fs";
import { childRepoViolations, MESSAGE } from "../../scripts/generated-child-repos.mjs";

const raw = await new Promise((resolve) => {
  let data = "";
  process.stdin.setEncoding("utf8");
  process.stdin.on("data", (chunk) => {
    data += chunk;
  });
  process.stdin.on("end", () => resolve(data));
});

if (!raw.trim()) process.exit(0);
let payload;
try {
  payload = JSON.parse(raw);
} catch {
  process.exit(0);
}
const file = String(payload.file_path ?? payload.filePath ?? "");
if (!/(repository\.(cs|ts|rs)\.tmpl|generated[/\\].*[Rr]epositor)/.test(file)) process.exit(0);
let text = "";
try {
  text = readFileSync(file, "utf8");
} catch {
  process.exit(0);
}
const hits = childRepoViolations(text);
if (hits.length) {
  console.error(`${file}: ${MESSAGE}`);
  for (const hit of hits) console.error(`${hit.line}: ${hit.text}`);
  process.exit(1);
}
