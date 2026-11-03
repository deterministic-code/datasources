import { existsSync, readFileSync, unlinkSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const repo = process.env.DATASOURCES_REPO ?? "/opt/repo";
const dir = process.argv[2] ?? ".";

const fileFor = (name) => {
  if (name === "@deterministic-code/datasources") return join(repo, "typescript/datasources");
  if (name === "@deterministic-code/datasources-code-generators") return join(repo, "typescript/code-generators");
  if (name === "@deterministic-code/datasources-cli") return join(repo, "cli");
  const prefix = "@deterministic-code/datasources-";
  if (name.startsWith(prefix)) return join(repo, `typescript/providers/${name.slice(prefix.length)}`);
  throw new Error(`e2e-abs-file-deps: no ${repo} path for ${name}`);
};

const rewrite = (deps) => {
  if (deps === undefined) return;
  for (const [name, spec] of Object.entries(deps)) {
    if (typeof spec === "string" && spec.startsWith("file:") && !spec.startsWith(`file:${repo}`)) {
      deps[name] = `file:${fileFor(name)}`;
    }
  }
};

const pkgPath = join(dir, "package.json");
if (!existsSync(pkgPath)) process.exit(0);
const pkg = JSON.parse(readFileSync(pkgPath, "utf8"));
rewrite(pkg.dependencies);
rewrite(pkg.devDependencies);
delete pkg.overrides;
writeFileSync(pkgPath, `${JSON.stringify(pkg, null, 2)}\n`);
const lock = join(dir, "package-lock.json");
if (existsSync(lock)) unlinkSync(lock);
