import { mkdirSync, writeFileSync } from "node:fs";
import { dirname } from "node:path";
import { run } from "./src/run.ts";

const log = process.env.E2E_MATRIX_LOG ?? "";
const lines = await run();
if (log) {
  mkdirSync(dirname(log), { recursive: true });
  writeFileSync(log, lines.length ? `${lines.join("\n")}\n` : "");
}
