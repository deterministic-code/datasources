import { spawnSync } from "node:child_process";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const dist = join(root, "docs/.vitepress/dist");
const branch = "gh-pages";

const run = (cmd, args, cwd = root) => {
  const result = spawnSync(cmd, args, { cwd, stdio: "inherit", encoding: "utf8" });
  if (result.status) process.exit(result.status ?? 1);
};

const capture = (cmd, args, cwd = root) => {
  const result = spawnSync(cmd, args, { cwd, encoding: "utf8" });
  if (result.status) {
    process.stderr.write(result.stderr ?? "");
    process.exit(result.status ?? 1);
  }
  return result.stdout.trim();
};

process.env.DOCS_BASE ??= "/datasources/";
run("npm", ["run", "docs:build"]);

const remote = capture("git", ["remote", "get-url", "origin"]);
run("git", ["init"], dist);
run("git", ["checkout", "-B", branch], dist);
run("git", ["add", "-A"], dist);
run("git", ["commit", "-m", "publish docs"], dist);
run("git", ["push", "--force", remote, `HEAD:${branch}`], dist);
console.log(`Published ${dist} to ${remote}#${branch}`);
