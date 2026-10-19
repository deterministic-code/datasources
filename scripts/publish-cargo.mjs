import { mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { spawnSync } from "node:child_process";

const repo = resolve(import.meta.dirname, "..");
const rust = join(repo, "rust");
const targetDir = process.env.CARGO_TARGET_DIR || join(rust, "target");
const packageDir = join(targetDir, "package");
const argv = process.argv.slice(2);
const doPublish = argv.includes("--publish");
const versionAt = argv.indexOf("--version");
const stamp = versionAt >= 0 ? argv[versionAt + 1] : undefined;
if (versionAt >= 0 && !stamp) {
  console.error("usage: npm run publish:cargo -- [--publish] [--version x.y.z]");
  process.exit(1);
}

const crates = [
  "datasources",
  "datasources-json",
  "datasources-memory",
  "datasources-sqlite",
  "datasources-xml",
  "datasources-yaml",
  "datasources-mysql",
  "datasources-sqlserver",
  "datasources-postgres",
  "datasources-oracle",
  "datasources-mongodb",
  "datasources-dynamodb",
];

const run = (command, args, cwd, opts = {}) => {
  const result = spawnSync(command, args, {
    cwd,
    encoding: "utf8",
    stdio: opts.capture ? ["ignore", "pipe", "pipe"] : "inherit",
    env: opts.env ?? process.env,
  });
  if (result.status !== 0 && !opts.allowFail) process.exit(result.status ?? 1);
  return result;
};

const tomlVersion = async () => {
  const text = await readFile(join(rust, "Cargo.toml"), "utf8");
  const match = text.match(/^version = "([^"]+)"/m);
  if (!match) {
    console.error("rust/Cargo.toml is missing workspace.package version");
    process.exit(1);
  }
  return match[1];
};

const stampVersion = async (ver) => {
  const path = join(rust, "Cargo.toml");
  const next = (await readFile(path, "utf8"))
    .replace(/^version = "[^"]+"/m, `version = "${ver}"`)
    .replace(
      /datasources = \{ path = "datasources", version = "[^"]+" \}/,
      `datasources = { path = "datasources", version = "${ver}" }`,
    );
  await writeFile(path, next);
};

const published = (name, ver) => {
  const result = run("cargo", ["info", "-q", `${name}@${ver}`], rust, {
    capture: true,
    allowFail: true,
  });
  return result.status === 0 && result.stdout.includes(ver);
};

const dirty = () => {
  const result = run("git", ["status", "--porcelain"], repo, { capture: true });
  return result.stdout.trim().length > 0;
};

const waitPublished = (name, ver) => {
  for (let i = 0; i < 36; i++) {
    if (published(name, ver)) return;
    Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, 10_000);
  }
  console.error(`${name}@${ver} is not visible on crates.io yet`);
  process.exit(1);
};

const packageCrate = (name, ver) => {
  const args = ["package", "-p", name, "--no-verify"];
  if (dirty()) args.push("--allow-dirty");
  console.log(`package ${name}`);
  const result = run("cargo", args, rust, { capture: true, allowFail: true });
  if (result.status === 0) {
    process.stdout.write(result.stdout);
    return true;
  }
  const text = `${result.stdout}${result.stderr}`;
  if (name !== "datasources" && text.includes("no matching package named `datasources`")) {
    console.log(`skip package ${name} until datasources@${ver} is on crates.io`);
    return false;
  }
  process.stdout.write(result.stdout);
  process.stderr.write(result.stderr);
  process.exit(result.status ?? 1);
};

const smoke = async (ver) => {
  const dir = await mkdtemp(join(tmpdir(), "datasources-cargo-"));
  try {
    run("cargo", ["new", "--lib", "smoke"], dir);
    const crate = join(packageDir, `datasources-${ver}`);
    run("tar", ["-xzf", join(packageDir, `datasources-${ver}.crate`), "-C", packageDir], rust);
    run("cargo", ["add", "datasources", "--path", crate], join(dir, "smoke"));
    run("cargo", ["check"], join(dir, "smoke"));
    console.log(`smoke ok (${dir})`);
  } finally {
    await rm(dir, { recursive: true, force: true });
  }
};

const publish = (name, ver) => {
  if (published(name, ver)) {
    console.log(`skip ${name}@${ver} (already on crates.io)`);
    return;
  }
  const args = ["publish", "-p", name];
  if (dirty()) args.push("--allow-dirty");
  console.log(`publish ${name}@${ver}`);
  const result = run("cargo", args, rust, { capture: true, allowFail: true });
  if (result.status !== 0) {
    process.stdout.write(result.stdout);
    process.stderr.write(result.stderr);
    process.exit(result.status ?? 1);
  }
  process.stdout.write(result.stdout);
};

if (stamp) await stampVersion(stamp);
const ver = await tomlVersion();
if (stamp && ver !== stamp) {
  console.error(`stamped ${stamp} but cargo metadata is ${ver}`);
  process.exit(1);
}

for (const name of crates.slice(1)) packageCrate(name, ver);
packageCrate("datasources", ver);
await smoke(ver);

if (!doPublish) {
  console.log(`dry-run packed ${ver}; pass --publish to push to crates.io`);
  process.exit(0);
}

if (!process.env.CARGO_REGISTRY_TOKEN) {
  console.error("CARGO_REGISTRY_TOKEN is not set; stop before push");
  process.exit(1);
}

publish("datasources", ver);
waitPublished("datasources", ver);
for (const name of crates.slice(1)) {
  packageCrate(name, ver);
  publish(name, ver);
}
