import { existsSync } from "node:fs";
import { cp, mkdir, mkdtemp, readFile, readdir, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { spawnSync } from "node:child_process";

const tsRoot = import.meta.dirname;
const repo = resolve(tsRoot, "..");
const cliRoot = join(repo, "cli");
const argv = process.argv.slice(2);
const doPublish = argv.includes("--publish");
const cliOnly = argv.includes("--cli-only");
const versionAt = argv.indexOf("--version");
const stamp = versionAt >= 0 ? argv[versionAt + 1] : undefined;
if (versionAt >= 0 && !stamp) {
  console.error("usage: npm run publish:npm -- [--publish] [--cli-only] [--version x.y.z]");
  process.exit(1);
}

const providers = [
  "json",
  "memory",
  "sqlite",
  "xml",
  "yaml",
  "mysql",
  "sqlserver",
  "postgres",
  "oracle",
  "mongodb",
  "dynamodb",
];
const sqlProviders = ["sqlite", "mysql", "postgres", "sqlserver", "oracle"];
const vendorNames = [
  "generators-common",
  "deterministic-generators-common",
  "typescript-generators",
  "csharp-generators",
  "rust-generators",
  "generators-openapi",
  "patch-merger",
  "deterministic-specifications-typescript",
];

const libraries = [
  { dir: join(tsRoot, "datasources"), name: "@deterministic-code/datasources" },
  ...providers.map((name) => ({
    dir: join(tsRoot, "providers", name),
    name: `@deterministic-code/datasources-${name}`,
  })),
];

const run = (command, args, cwd, opts = {}) => {
  const result = spawnSync(command, args, {
    cwd,
    encoding: "utf8",
    stdio: opts.capture ? ["ignore", "pipe", "pipe"] : "inherit",
  });
  if (result.status !== 0 && !opts.allowFail) process.exit(result.status ?? 1);
  return result;
};

const published = (name, version) => {
  const result = run("npm", ["view", `${name}@${version}`, "version"], repo, {
    capture: true,
    allowFail: true,
  });
  return result.status === 0 && result.stdout.trim() === version;
};

const assertVendoredOnMain = async () => {
  const lock = JSON.parse(await readFile(join(cliRoot, "package-lock.json"), "utf8"));
  const stale = [];
  for (const [path, meta] of Object.entries(lock.packages ?? {})) {
    if (!/^node_modules\/@deterministic-code\/[^/]+$/.test(path)) continue;
    const resolved = String(meta.resolved ?? "");
    const match = resolved.match(/github\.com\/deterministic-code\/(.+?)\.git#([0-9a-f]{40})$/);
    if (!match) continue;
    const [, name, sha] = match;
    const remote = run(
      "git",
      ["ls-remote", `https://github.com/deterministic-code/${name}.git`, "refs/heads/main"],
      repo,
      { capture: true, allowFail: true },
    );
    const head = (remote.stdout ?? "").trim().split(/\s+/)[0] ?? "";
    if (remote.status !== 0 || head.length !== 40) {
      console.error(`cannot read main for ${name}`);
      process.exit(remote.status ?? 1);
    }
    if (head !== sha)
      stale.push(`${path} ${sha.slice(0, 7)} is behind main ${head.slice(0, 7)}`);
  }
  if (stale.length === 0) return;
  console.error("bundled generator commits are behind GitHub main:");
  for (const line of stale) console.error(`  ${line}`);
  console.error("run npm update in cli/, commit package-lock.json, and publish a new CLI version");
  process.exit(1);
};

const rewriteExports = (value) => {
  if (typeof value === "string")
    return value.replaceAll("/src/", "/dist/").replace(/\.ts$/, ".js");
  if (Array.isArray(value)) return value.map(rewriteExports);
  if (value && typeof value === "object") {
    return Object.fromEntries(
      Object.entries(value).map(([key, next]) => [key, rewriteExports(next)]),
    );
  }
  return value;
};

const pinWorkspace = (deps, version) => {
  if (!deps) return;
  for (const [name, spec] of Object.entries(deps)) {
    if (
      (name === "@deterministic-code/datasources" ||
        name.startsWith("@deterministic-code/datasources-")) &&
      (spec === "workspace:*" || spec === "*" || String(spec).startsWith("file:"))
    ) {
      deps[name] = `^${version}`;
    }
  }
};

const pinGithub = (deps, versions) => {
  if (!deps) return;
  for (const [name, spec] of Object.entries(deps)) {
    if (!String(spec).startsWith("github:")) continue;
    const version = versions[name];
    if (version) deps[name] = version;
  }
};

const walk = async (dir) => {
  const files = [];
  for (const entry of await readdir(dir, { withFileTypes: true })) {
    const path = join(dir, entry.name);
    if (entry.isDirectory()) {
      if (entry.name === "node_modules" || entry.name === "dist") continue;
      files.push(...(await walk(path)));
    } else if (entry.name.endsWith(".ts") && !entry.name.endsWith(".test.ts")) {
      files.push(path);
    }
  }
  return files;
};

const compileCli = async () => {
  const { build } = await import(join(cliRoot, "node_modules/esbuild/lib/main.js"));
  const rewriteTs = {
    name: "rewrite-ts",
    setup(result) {
      result.onLoad({ filter: /\.ts$/ }, async (args) => ({
        contents: (await readFile(args.path, "utf8")).replace(
          /(["'])(\.[^"']+?)\.ts\1/g,
          "$1$2.js$1",
        ),
        loader: "ts",
      }));
    },
  };
  const src = join(cliRoot, "src");
  await build({
    entryPoints: await walk(src),
    outdir: join(cliRoot, "dist"),
    outbase: src,
    format: "esm",
    platform: "node",
    packages: "external",
    plugins: [rewriteTs],
    logLevel: "warning",
  });
};

const stageLibrary = async (pkg, version) => {
  const source = JSON.parse(await readFile(join(pkg.dir, "package.json"), "utf8"));
  const staged = {
    name: pkg.name,
    version,
    type: "module",
    main: rewriteExports(source.main ?? "./src/index.ts"),
    types: rewriteExports(source.types ?? "./dist/index.d.ts"),
    exports: rewriteExports(source.exports),
    files: ["dist"],
    publishConfig: { access: "public" },
    dependencies: source.dependencies ? { ...source.dependencies } : undefined,
  };
  pinWorkspace(staged.dependencies, version);
  delete staged.dependencies?.["@deterministic-code/datasources-code-generators"];
  const dest = await mkdtemp(join(tmpdir(), "ds-lib-"));
  await writeFile(join(dest, "package.json"), `${JSON.stringify(staged, null, 2)}\n`);
  await cp(join(pkg.dir, "dist"), join(dest, "dist"), { recursive: true });
  return dest;
};

const vendorVersions = async () => {
  const versions = {};
  for (const name of vendorNames) {
    const manifest = JSON.parse(
      await readFile(join(cliRoot, "node_modules/@deterministic-code", name, "package.json"), "utf8"),
    );
    versions[manifest.name] = manifest.version;
  }
  const codegen = JSON.parse(
    await readFile(join(tsRoot, "code-generators", "package.json"), "utf8"),
  );
  versions[codegen.name] = stamp ?? codegen.version;
  return versions;
};

const rewriteVendorManifest = async (dir, versions, version) => {
  const path = join(dir, "package.json");
  const manifest = JSON.parse(await readFile(path, "utf8"));
  pinWorkspace(manifest.dependencies, version);
  pinGithub(manifest.dependencies, versions);
  pinWorkspace(manifest.devDependencies, version);
  pinGithub(manifest.devDependencies, versions);
  delete manifest.scripts;
  delete manifest.devDependencies;
  if (manifest.exports) manifest.exports = rewriteExports(manifest.exports);
  if (manifest.main) manifest.main = rewriteExports(manifest.main);
  if (manifest.types) manifest.types = rewriteExports(manifest.types);
  await writeFile(path, `${JSON.stringify(manifest, null, 2)}\n`);
};

const stageCli = async (version, versions, libraryVersion = version) => {
  const dest = await mkdtemp(join(tmpdir(), "ds-cli-"));
  const codegenSrc = join(tsRoot, "code-generators");
  const codegenDest = join(dest, "node_modules/@deterministic-code/datasources-code-generators");
  await mkdir(dirname(codegenDest), { recursive: true });
  await cp(join(codegenSrc, "dist"), join(codegenDest, "dist"), { recursive: true });
  await writeFile(
    join(codegenDest, "package.json"),
    `${JSON.stringify(
      {
        name: "@deterministic-code/datasources-code-generators",
        version: versions["@deterministic-code/datasources-code-generators"],
        type: "module",
        exports: rewriteExports(
          JSON.parse(await readFile(join(codegenSrc, "package.json"), "utf8")).exports,
        ),
        dependencies: {
          "@deterministic-code/datasources": `^${libraryVersion}`,
          "@deterministic-code/datasources-mysql": `^${libraryVersion}`,
          "@deterministic-code/datasources-oracle": `^${libraryVersion}`,
          "@deterministic-code/datasources-postgres": `^${libraryVersion}`,
          "@deterministic-code/datasources-sqlite": `^${libraryVersion}`,
          "@deterministic-code/datasources-sqlserver": `^${libraryVersion}`,
          "@deterministic-code/deterministic-generators-common":
            versions["@deterministic-code/deterministic-generators-common"],
          "@deterministic-code/generators-common": versions["@deterministic-code/generators-common"],
          pluralize: "^8.0.0",
          yaml: "^2.6.0",
        },
      },
      null,
      2,
    )}\n`,
  );

  const detRoot = join(dest, "node_modules/@deterministic-code");
  await mkdir(detRoot, { recursive: true });
  for (const name of vendorNames) {
    const from = join(cliRoot, "node_modules/@deterministic-code", name);
    const to = join(detRoot, name);
    await cp(from, to, { recursive: true });
    await rm(join(to, "node_modules"), { recursive: true, force: true });
    await rewriteVendorManifest(to, versions, libraryVersion);
  }

  await mkdir(join(dest, "bin"), { recursive: true });
  await writeFile(
    join(dest, "bin/datasources.mjs"),
    `#!/usr/bin/env node\nimport "../dist/main.js";\n`,
  );
  await cp(join(cliRoot, "dist"), join(dest, "dist"), { recursive: true });

  const bundled = [
    "@deterministic-code/datasources-code-generators",
    ...vendorNames.map((name) => `@deterministic-code/${name}`),
  ];
  const dependencies = {
    "@deterministic-code/datasources-code-generators": versions["@deterministic-code/datasources-code-generators"],
    ...Object.fromEntries(vendorNames.map((name) => [`@deterministic-code/${name}`, versions[`@deterministic-code/${name}`]])),
    "@deterministic-code/datasources": `^${libraryVersion}`,
    ...Object.fromEntries(sqlProviders.map((name) => [`@deterministic-code/datasources-${name}`, `^${libraryVersion}`])),
    "@inquirer/prompts": "^7.8.0",
    yaml: "^2.6.0",
    mustache: "^4.2.0",
    pluralize: "^8.0.0",
    ajv: "^8.18.0",
    "fast-xml-parser": "^5.2.5",
  };
  await writeFile(
    join(dest, "package.json"),
    `${JSON.stringify(
      {
        name: "@deterministic-code/datasources-cli",
        version,
        type: "module",
        bin: { datasources: "./bin/datasources.mjs" },
        files: ["bin", "dist"],
        publishConfig: { access: "public" },
        dependencies,
        bundledDependencies: bundled,
      },
      null,
      2,
    )}\n`,
  );
  return dest;
};

const smoke = async (tarballs) => {
  const dir = await mkdtemp(join(tmpdir(), "ds-smoke-"));
  run("npm", ["init", "-y"], dir);
  const missing = tarballs.filter((path) => !existsSync(path));
  if (missing.length) {
    console.error("missing packed tarballs", missing);
    process.exit(1);
  }
  run("npm", ["install", ...tarballs], dir);
  const check = run(
    process.execPath,
    [
      "--input-type=module",
      "-e",
      `import { Datasource } from "@deterministic-code/datasources";
       import { SqliteDriver } from "@deterministic-code/datasources-sqlite";
       if (typeof Datasource !== "function") throw new Error("Datasource missing");
       if (typeof SqliteDriver !== "function") throw new Error("SqliteDriver missing");
       console.log("ok", Datasource.name, SqliteDriver.name);`,
    ],
    dir,
  );
  if (check.status !== 0) process.exit(check.status ?? 1);
  run(join(dir, "node_modules/.bin/datasources"), ["--help"], dir);
  await rm(dir, { recursive: true, force: true });
};

const smokeCli = async (tarball) => {
  const dir = await mkdtemp(join(tmpdir(), "ds-cli-smoke-"));
  run("npm", ["init", "-y"], dir);
  if (!existsSync(tarball)) {
    console.error("missing packed tarball", tarball);
    process.exit(1);
  }
  run("npm", ["install", tarball], dir);
  run(join(dir, "node_modules/.bin/datasources"), ["--help"], dir);
  await rm(dir, { recursive: true, force: true });
};

const packName = (name, version) =>
  `${name.replace("@", "").replace("/", "-")}-${version}.tgz`;

if (!cliOnly) await assertVendoredOnMain();

if (!cliOnly) {
  console.log("building typescript packages");
  run("npm", ["run", "build"], tsRoot);
}
console.log("building cli generators");
run(process.execPath, [join(cliRoot, "scripts/build-generators.mjs")], cliRoot);
console.log("compiling cli");
await compileCli();

const versions = await vendorVersions();
const publishedCore = run("npm", ["view", "@deterministic-code/datasources", "version"], repo, {
  capture: true,
  allowFail: true,
});
const libraryVersion =
  stamp ??
  (cliOnly && publishedCore.status === 0 && publishedCore.stdout.trim()
    ? publishedCore.stdout.trim()
    : JSON.parse(await readFile(join(tsRoot, "datasources/package.json"), "utf8")).version);
const cliVersion = stamp ?? JSON.parse(await readFile(join(cliRoot, "package.json"), "utf8")).version;

const staged = [];
if (!cliOnly) {
  for (const pkg of libraries) {
    const version = stamp ?? JSON.parse(await readFile(join(pkg.dir, "package.json"), "utf8")).version;
    console.log(`staging ${pkg.name}@${version}`);
    staged.push({ ...pkg, version, dest: await stageLibrary(pkg, version) });
  }
}
console.log(`staging @deterministic-code/datasources-cli@${cliVersion}`);
staged.push({
  dir: cliRoot,
  name: "@deterministic-code/datasources-cli",
  version: cliVersion,
  dest: await stageCli(
    cliVersion,
    { ...versions, "@deterministic-code/datasources-code-generators": stamp ?? versions["@deterministic-code/datasources-code-generators"] },
    libraryVersion,
  ),
});

const packs = await mkdtemp(join(tmpdir(), "ds-pack-"));
const tarballs = [];
for (const pkg of staged) {
  console.log(`packing ${pkg.name}@${pkg.version}`);
  run("npm", ["pack", "--pack-destination", packs], pkg.dest);
  tarballs.push(join(packs, packName(pkg.name, pkg.version)));
}

console.log("smoke-testing packed tarballs");
if (cliOnly) await smokeCli(tarballs[0]);
else await smoke(tarballs);

if (!doPublish) {
  console.log("dry-run complete; pass --publish to release");
  for (const pkg of staged) await rm(pkg.dest, { recursive: true, force: true });
  await rm(packs, { recursive: true, force: true });
  process.exit(0);
}

const org = run("npm", ["org", "ls", "deterministic-code"], repo, { capture: true, allowFail: true });
if (org.status !== 0) {
  console.error(
    "cannot publish: this login cannot access the @deterministic-code npm org",
  );
  process.exit(1);
}

for (const pkg of staged) {
  if (published(pkg.name, pkg.version)) {
    console.log(`skip ${pkg.name}@${pkg.version} (already on npm)`);
    continue;
  }
  console.log(`publishing ${pkg.name}@${pkg.version}`);
  const result = run("npm", ["publish", "--access", "public"], pkg.dest, {
    allowFail: true,
    capture: true,
  });
  if (result.stdout) process.stdout.write(result.stdout);
  if (result.stderr) process.stderr.write(result.stderr);
  const err = `${result.stderr ?? ""}${result.stdout ?? ""}`;
  if (result.status === 0) continue;
  if (result.status !== 0 && /previously staged version|cannot publish over existing version/i.test(err)) {
    console.log(`skip ${pkg.name}@${pkg.version} (already staged or published)`);
    continue;
  }
  process.exit(result.status ?? 1);
}

for (const pkg of staged) await rm(pkg.dest, { recursive: true, force: true });
await rm(packs, { recursive: true, force: true });
