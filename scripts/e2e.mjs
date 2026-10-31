import { spawnSync } from "node:child_process";
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const root = dirname(fileURLToPath(new URL(".", import.meta.url)));
const cli = join(root, "cli/bin/datasources.mjs");
const composeFile = join(root, "docker/compose.yml");
const skipDocker = Boolean(process.env.SKIP_DOCKER);
const sqliteOnly = process.env.E2E_SQLITE === "1" || process.argv.includes("--sqlite");
const dialects = ["sqlite", "mysql", "postgres"];
const apply = skipDocker || sqliteOnly ? ["sqlite"] : dialects;
const ecosystems = { ts: "typescript", rust: "rust", csharp: "csharp" };
const samples = [
  { name: "user-roles-simple-cli", spec: "samples/user-roles-simple-cli-shared/deterministic" },
  { name: "contacts-crud-enriched-medium-rest", spec: "samples/contacts-crud-enriched-medium-rest-shared/deterministic" },
  { name: "contacts-simple-cli", spec: "quick_starts/contacts_cli/shared/deterministic" },
];

const mysqlUrl = () => process.env.MYSQL_URL || "mysql://ds:ds@127.0.0.1:3306/datasources";
const postgresUrl = () => process.env.POSTGRES_URL || "postgres://ds:ds@127.0.0.1:5432/datasources";

const requireOk = (result, label) => {
  if ((result.status ?? 1) !== 0)
    throw new Error(`${label}: ${result.stderr || result.stdout || "failed"}`);
  return result;
};

const spawn = (command, args, cwd = root, env = {}) =>
  spawnSync(command, args, { cwd, encoding: "utf8", stdio: "inherit", env: { ...process.env, ...env } });

const run = (command, args, cwd = root, env = {}) =>
  requireOk(spawn(command, args, cwd, env), `${command} ${args.join(" ")}`);

const collect = (failed, label, result) => {
  if ((result.status ?? 1) !== 0) failed.push(label);
};

const compose = (...args) =>
  spawnSync("docker", ["compose", "-p", "datasources", "-f", composeFile, ...args], {
    cwd: root,
    encoding: "utf8",
    stdio: "inherit",
  });

const leftover = () => {
  const listed = spawnSync("docker", ["ps", "-aq", "--filter", "name=datasources"], { encoding: "utf8" });
  const ids = (listed.stdout || "").trim().split(/\s+/).filter(Boolean);
  if (ids.length) spawnSync("docker", ["rm", "-f", ...ids], { encoding: "utf8", stdio: "inherit" });
};

const up = () => {
  if (skipDocker) return;
  requireOk(compose("up", "-d", "--wait"), "compose up");
};

const down = () => {
  if (skipDocker) return;
  compose("down", "-v", "--remove-orphans");
  leftover();
};

const sampleDir = (name, ecosystem) => join(root, "samples", ecosystems[ecosystem], name);

const datasources = (cwd, args) => run(process.execPath, [cli, ...args], cwd);

const sqliteFile = (cwd) => {
  const path = join(cwd, "app.sqlite");
  mkdirSync(cwd, { recursive: true });
  writeFileSync(path, "", { flag: "a" });
  return path;
};

const connection = (cwd, provider) =>
  ({
    sqlite: sqliteFile(cwd),
    mysql: mysqlUrl(),
    postgres: postgresUrl(),
  })[provider];

const migraters = (cwd, command, provider, extra = []) =>
  run("npx", [command, "--provider", provider, "--connection", connection(cwd, provider), ...extra], cwd);

const migrateUp = (cwd, provider) => {
  migraters(cwd, "migrate-setup", provider);
  migraters(cwd, "migrate-up", provider, ["--migrations-root", "sql"]);
};

const setupProject = (cwd, ecosystem) => {
  writeFileSync(
    join(cwd, ".env"),
    `INSTALLED_PROVIDERS=${dialects.join(",")}\nPROVIDER=sqlite\nDATASOURCE_URL=sqlite:///:memory:\n`,
  );
  datasources(cwd, ["setup", "--yes", "--languages", ecosystem]);
  const sample = samples.find((item) => cwd.endsWith(item.name));
  if (!sample) throw new Error(`no spec for ${cwd}`);
  const tsHome = sampleDir(sample.name, "ts");
  datasources(tsHome, ["migrations", "create", "--cwd", cwd, "--input", join(root, sample.spec)]);
  if (ecosystem === "ts") migrateUp(cwd, "sqlite");
};

const cargoEnv = () => {
  const { CARGO_TARGET_DIR: _drop, ...env } = process.env;
  return env;
};

const tests = () => {
  const failed = [];
  if (sqliteOnly || skipDocker) {
    collect(failed, "ts user-roles", spawn("npm", ["run", "test:e2e", "--prefix", "samples/typescript/user-roles-simple-cli"]));
    collect(failed, "ts contacts rest sqlite", spawn("npm", ["run", "test:e2e:contacts:rest:ts:sqlite"]));
    collect(failed, "ts contacts console sqlite", spawn("npm", ["run", "test:e2e:contacts:console:ts:sqlite"]));
    collect(
      failed,
      "rust user-roles sqlite",
      spawn("cargo", ["test", "--manifest-path", "samples/rust/user-roles-simple-cli/Cargo.toml", "--test", "e2e", "sqlite", "--", "--test-threads=1"], root, cargoEnv()),
    );
    collect(failed, "rust contacts rest sqlite", spawn("npm", ["run", "test:e2e:contacts:rest:rust:sqlite"]));
    collect(
      failed,
      "csharp user-roles sqlite",
      spawn("dotnet", ["test", "samples/csharp/user-roles-simple-cli/UserRolesSimpleCli.Tests.csproj", "--filter", "Category=e2e&FullyQualifiedName~Sqlite", "--p:CollectCoverage=false"]),
    );
    collect(failed, "csharp contacts rest sqlite", spawn("npm", ["run", "test:e2e:contacts:rest:csharp:sqlite"]));
  } else {
    collect(failed, "test:e2e:ci", spawn("npm", ["run", "test:e2e:ci"]));
    collect(failed, "test:e2e:rust", spawn("npm", ["run", "test:e2e:rust"]));
    collect(failed, "test:e2e:csharp", spawn("npm", ["run", "test:e2e:csharp"]));
    collect(failed, "test:e2e:samples", spawn("npm", ["run", "test:e2e:samples"]));
  }
  if (failed.length) throw new Error(`failed: ${failed.join(", ")}`);
};

try {
  up();
  for (const sample of samples)
    for (const ecosystem of Object.keys(ecosystems)) setupProject(sampleDir(sample.name, ecosystem), ecosystem);
  const first = sampleDir(samples[0].name, "ts");
  for (const dialect of apply.filter((name) => name !== "sqlite")) migrateUp(first, dialect);
  tests();
} finally {
  down();
}
