import { cp, mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { spawnSync } from "node:child_process";

const root = import.meta.dirname;
const dry = process.argv.includes("--dry-run");

const packages = [
  {
    dir: join(root, "datasources"),
    name: "@deterministic-code/datasources",
  },
  {
    dir: join(root, "providers/mysql"),
    name: "@deterministic-code/datasources-mysql",
  },
  {
    dir: join(root, "providers/postgres"),
    name: "@deterministic-code/datasources-postgres",
  },
  {
    dir: join(root, "providers/sqlite"),
    name: "@deterministic-code/datasources-sqlite",
  },
];

const run = (command, args, cwd) => {
  const result = spawnSync(command, args, { cwd, stdio: "inherit" });
  if (result.status !== 0) process.exit(result.status ?? 1);
};

run("npm", ["run", "build"], root);

for (const pkg of packages) {
  const manifest = JSON.parse(await readFile(join(pkg.dir, "package.json"), "utf8"));
  manifest.name = pkg.name;
  if (manifest.dependencies?.["@datasources/core"]) {
    manifest.dependencies["@deterministic-code/datasources"] =
      `^${manifest.version}`;
    delete manifest.dependencies["@datasources/core"];
  }
  delete manifest.devDependencies;
  delete manifest.scripts;

  const staging = await mkdtemp(join(tmpdir(), "ds-publish-"));
  await writeFile(join(staging, "package.json"), `${JSON.stringify(manifest, null, 2)}\n`);
  await cp(join(pkg.dir, "src"), join(staging, "src"), { recursive: true });
  await cp(join(pkg.dir, "dist"), join(staging, "dist"), { recursive: true });

  console.log(`\n=== ${pkg.name}@${manifest.version} ===`);
  run("npm", ["publish", "--access", "public", ...(dry ? ["--dry-run"] : [])], staging);
  await rm(staging, { recursive: true, force: true });
}
