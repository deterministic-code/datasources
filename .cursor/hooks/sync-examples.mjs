#!/usr/bin/env node
import { syncExamples } from "../../scripts/sync-examples.mjs";

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
if (!/\.(cs|rs|ts)$/.test(file)) process.exit(0);
const { errors } = syncExamples({ files: [file], edits: payload.edits ?? [] });
if (errors.length) console.error(errors.join("\n"));
process.exit(0);
