import { spawnSync } from "node:child_process";
import { cp, mkdir, readFile, readdir, rm } from "node:fs/promises";
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
const cargoToml = await readFile(join(repo, "rust/Cargo.toml"), "utf8");
const rustVersion = cargoToml.match(/^version = "([^"]+)"/m)?.[1];
if (!rustVersion) {
  console.error("rust/Cargo.toml is missing workspace.package version");
  process.exit(1);
}
const keepCrate = (stem) =>
  stem === `datasources-${rustVersion}` ||
  stem === `datasources-sqlite-${rustVersion}` ||
  stem === `datasources-mysql-${rustVersion}` ||
  stem === `datasources-postgres-${rustVersion}`;
for (const name of await readdir(packageDir).catch(() => [])) {
  if (!name.endsWith(".crate")) continue;
  const stem = name.slice(0, -".crate".length);
  if (!keepCrate(stem)) continue;
  run("tar", ["-xzf", join(packageDir, name), "-C", crates]);
}
