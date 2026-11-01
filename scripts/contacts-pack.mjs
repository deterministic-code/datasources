import { spawnSync } from "node:child_process";
import { cp, mkdir, readdir, rm } from "node:fs/promises";
import { join, resolve } from "node:path";

const repo = resolve(import.meta.dirname, "..");
const destCli = join(repo, "dest-cli");

const run = (command, args, cwd = repo) => {
  const result = spawnSync(command, args, { cwd, stdio: "inherit", encoding: "utf8" });
  if (result.status !== 0) process.exit(result.status ?? 1);
};

run(process.execPath, [join(repo, "typescript/publish-npm.mjs"), "--pack-destination", destCli]);
run(process.execPath, [join(repo, "scripts/publish-nuget.mjs")]);
run(process.execPath, [join(repo, "scripts/publish-cargo.mjs")]);

const nuget = join(destCli, "nuget");
await rm(nuget, { recursive: true, force: true });
await mkdir(nuget, { recursive: true });
const artifacts = join(repo, "csharp/artifacts");
for (const name of await readdir(artifacts)) {
  if (name.endsWith(".nupkg") && !name.endsWith(".snupkg"))
    await cp(join(artifacts, name), join(nuget, name));
}

const crates = join(destCli, "crates");
await rm(crates, { recursive: true, force: true });
await mkdir(crates, { recursive: true });
const packageDir = join(process.env.CARGO_TARGET_DIR || join(repo, "rust/target"), "package");
const keepCrate = (stem) =>
  /^datasources-\d/.test(stem) ||
  /^datasources-sqlite-\d/.test(stem) ||
  /^datasources-mysql-\d/.test(stem) ||
  /^datasources-postgres-\d/.test(stem);
for (const name of await readdir(packageDir).catch(() => [])) {
  if (!name.endsWith(".crate")) continue;
  const stem = name.slice(0, -".crate".length);
  if (!keepCrate(stem)) continue;
  run("tar", ["-xzf", join(packageDir, name), "-C", crates]);
}
