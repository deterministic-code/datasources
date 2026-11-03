#!/usr/bin/env node
import { createRequire } from "node:module";
import { spawn } from "node:child_process";
import { dirname, join } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const main = join(dirname(fileURLToPath(import.meta.url)), "../src/main.ts");

const tsx = () => {
  try {
    return import.meta.resolve("tsx");
  } catch {
    return pathToFileURL(createRequire(join(process.cwd(), "package.json")).resolve("tsx")).href;
  }
};

const child = spawn(process.execPath, ["--import", tsx(), main, ...process.argv.slice(2)], { stdio: "inherit" });
child.on("exit", (code) => process.exit(code ?? 1));
