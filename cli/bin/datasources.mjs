#!/usr/bin/env node
import { spawn } from "node:child_process";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const main = join(dirname(fileURLToPath(import.meta.url)), "../src/main.ts");
const child = spawn(
  process.execPath,
  ["--experimental-strip-types", main, ...process.argv.slice(2)],
  { stdio: "inherit" },
);
child.on("exit", (code) => process.exit(code ?? 1));
