import { spawnSync } from "node:child_process";
import { createRequire } from "node:module";
import {
  cpSync,
  existsSync,
  mkdirSync,
  mkdtempSync,
  readdirSync,
  readFileSync,
  rmSync,
  statSync,
  writeFileSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const repo = dirname(here);
const { parse } = createRequire(join(repo, "cli/package.json"))("yaml");
const cliBin = join(repo, "cli/bin/datasources.mjs");
const composeFile = join(repo, "docker/compose.yml");
const keep = process.env.KEEP === "1";
const source = flag("source") || "local";
const onlyProvider = flag("provider");

const nuget = {
  sqlite: "Datasources.Sqlite",
  mysql: "Datasources.Mysql",
  postgres: "Datasources.Postgres",
  sqlserver: "Datasources.SqlServer",
  oracle: "Datasources.Oracle",
  json: "Datasources.Json",
  memory: "Datasources.Memory",
  xml: "Datasources.Xml",
  yaml: "Datasources.Yaml",
  mongodb: "Datasources.MongoDb",
  dynamodb: "Datasources.DynamoDb",
};
const csharpProject = {
  Datasources: join(repo, "csharp/Datasources/Datasources.csproj"),
  "Datasources.Sqlite": join(repo, "csharp/Providers/Sqlite/Datasources.Sqlite.csproj"),
  "Datasources.Mysql": join(repo, "csharp/Providers/Mysql/Datasources.Mysql.csproj"),
  "Datasources.Postgres": join(repo, "csharp/Providers/Postgres/Datasources.Postgres.csproj"),
  "Datasources.SqlServer": join(repo, "csharp/Providers/SqlServer/Datasources.SqlServer.csproj"),
  "Datasources.Oracle": join(repo, "csharp/Providers/Oracle/Datasources.Oracle.csproj"),
  "Datasources.Json": join(repo, "csharp/Providers/Json/Datasources.Json.csproj"),
  "Datasources.Memory": join(repo, "csharp/Providers/Memory/Datasources.Memory.csproj"),
  "Datasources.Xml": join(repo, "csharp/Providers/Xml/Datasources.Xml.csproj"),
  "Datasources.Yaml": join(repo, "csharp/Providers/Yaml/Datasources.Yaml.csproj"),
  "Datasources.MongoDb": join(repo, "csharp/Providers/MongoDb/Datasources.MongoDb.csproj"),
  "Datasources.DynamoDb": join(repo, "csharp/Providers/DynamoDb/Datasources.DynamoDb.csproj"),
};
const npmPath = {
  "@deterministic-code/datasources": join(repo, "typescript/datasources"),
  "@deterministic-code/datasources-sqlite": join(repo, "typescript/providers/sqlite"),
  "@deterministic-code/datasources-mysql": join(repo, "typescript/providers/mysql"),
  "@deterministic-code/datasources-postgres": join(repo, "typescript/providers/postgres"),
};
const cargoPath = {
  datasources: join(repo, "rust/datasources"),
  "datasources-sqlite": join(repo, "rust/providers/sqlite"),
  "datasources-mysql": join(repo, "rust/providers/mysql"),
  "datasources-postgres": join(repo, "rust/providers/postgres"),
};

function flag(name) {
  const i = process.argv.indexOf(`--${name}`);
  return i >= 0 ? process.argv[i + 1] : undefined;
}

const mysqlUrl = () => process.env.MYSQL_URL || "mysql://ds:ds@127.0.0.1:3306/datasources";
const postgresUrl = () => process.env.POSTGRES_URL || "postgres://ds:ds@127.0.0.1:5432/datasources";

const fail = (label, result) => {
  throw new Error(`${label}: ${result.stderr || result.stdout || `exit ${result.status}`}`);
};

const run = (argv, cwd, env, io = "inherit") => {
  const [command, ...args] = argv;
  const result = spawnSync(command, args, {
    cwd,
    encoding: "utf8",
    stdio: io === "pipe" ? ["ignore", "pipe", "pipe"] : "inherit",
    env: { ...process.env, ...env },
  });
  if ((result.status ?? 1) !== 0) fail(argv.join(" "), result);
  return result;
};

const words = (line) => line.trim().split(/\s+/).filter(Boolean);

const discover = (dir, out = []) => {
  for (const name of readdirSync(dir)) {
    const path = join(dir, name);
    if (statSync(path).isDirectory()) {
      if (name === "frontend" || name === "node_modules") continue;
      discover(path, out);
    } else if (name === "quick_start.yaml") out.push(path);
  }
  return out;
};

const load = (path) => {
  const spec = parse(readFileSync(path, "utf8"));
  if (!spec?.name || !spec.language || !spec.providers?.length) throw new Error(`${path}: name, language, providers required`);
  return { path, dir: dirname(path), ...spec };
};

const rewrite = (argv, spec) => {
  if (source !== "local") return argv;
  if (argv[0] === "npx" && argv.includes("@deterministic-code/datasources-cli")) {
    const rest = argv.slice(argv.indexOf("@deterministic-code/datasources-cli") + 1);
    return [process.execPath, cliBin, ...rest];
  }
  if (argv[0] === "dotnet" && argv[1] === "add" && argv[2] === "package") {
    const id = argv[3];
    const ids = id === "Datasources" ? ["Datasources", ...spec.providers.map((p) => nuget[p]).filter(Boolean)] : [id];
    const refs = ids.map((name) => csharpProject[name]).filter(Boolean);
    if (!refs.length) throw new Error(`no local project for ${id}`);
    return ["dotnet", "add", "reference", ...refs];
  }
  if (argv[0] === "npm" && argv[1] === "install") {
    return ["npm", "install", ...argv.slice(2).map((pkg) => (npmPath[pkg] ? `file:${npmPath[pkg]}` : pkg))];
  }
  if (argv[0] === "cargo" && argv[1] === "add") {
    const crate = argv[2];
    return cargoPath[crate] ? ["cargo", "add", "--path", cargoPath[crate]] : argv;
  }
  return argv;
};

const copySpec = (spec, dest) => {
  for (const item of spec.copy ?? []) {
    const from = resolve(spec.dir, item.from);
    const to = resolve(dest, item.to);
    if (!existsSync(from)) throw new Error(`copy missing: ${from}`);
    mkdirSync(statSync(from).isDirectory() ? to : dirname(to), { recursive: true });
    cpSync(from, to, { recursive: true });
  }
  const settings = join(dest, "deterministic/settings.yaml");
  if (!existsSync(settings)) {
    mkdirSync(dirname(settings), { recursive: true });
    writeFileSync(
      settings,
      "settings:\n  backend:\n    datasources: sqlite,mysql,postgres\n",
    );
  }
};

const compose = (...args) =>
  spawnSync("docker", ["compose", "-p", "datasources", "-f", composeFile, ...args], {
    cwd: repo,
    encoding: "utf8",
    stdio: ["ignore", "pipe", "pipe"],
  });

const sqlClient = (provider, sql) => {
  if (provider === "mysql") {
    const url = new URL(mysqlUrl());
    return spawnSync(
      "mysql",
      ["-h", url.hostname, "-P", url.port || "3306", "-u", decodeURIComponent(url.username), `-p${decodeURIComponent(url.password)}`, "-e", sql],
      { encoding: "utf8", stdio: ["ignore", "pipe", "pipe"] },
    );
  }
  return spawnSync("psql", [postgresUrl(), "-c", sql], { encoding: "utf8", stdio: ["ignore", "pipe", "pipe"] });
};

const sqlDocker = (provider, sql) =>
  provider === "mysql"
    ? compose("exec", "-T", "mysql", "mysql", "-uds", "-pds", "-e", sql)
    : compose("exec", "-T", "postgres", "psql", "-U", "ds", "-d", "datasources", "-c", sql);

const sqlExec = (provider, sql) => {
  const local = sqlClient(provider, sql);
  if ((local.status ?? 1) === 0) return;
  const docked = sqlDocker(provider, sql);
  if ((docked.status ?? 1) !== 0) fail(`${provider} ${sql}`, docked.status != null ? docked : local);
};

const isolate = (provider, dir) => {
  if (provider === "sqlite") return { url: `sqlite:///${join(dir, "app.sqlite")}`, drop: () => {} };
  const name = `qs_${crypto.randomUUID().replaceAll("-", "").slice(0, 16)}`;
  if (provider === "mysql") {
    sqlExec("mysql", `CREATE DATABASE \`${name}\``);
    const url = new URL(mysqlUrl());
    url.pathname = `/${name}`;
    return { url: url.toString(), drop: () => sqlExec("mysql", `DROP DATABASE IF EXISTS \`${name}\``) };
  }
  sqlExec("postgres", `CREATE SCHEMA ${name}`);
  const base = postgresUrl();
  return {
    url: `${base}${base.includes("?") ? "&" : "?"}schema=${name}`,
    drop: () => sqlExec("postgres", `DROP SCHEMA IF EXISTS ${name} CASCADE`),
  };
};

const normalize = (text) => text.replace(/\r\n/g, "\n").replace(/\s+$/, "") + "\n";

const expectedOf = (spec, provider) => {
  const named = join(spec.dir, `expected.${provider}.txt`);
  const fallback = join(spec.dir, spec.expected || "expected.txt");
  const path = existsSync(named) ? named : fallback;
  if (!existsSync(path)) throw new Error(`${spec.path}: missing ${path}`);
  return normalize(readFileSync(path, "utf8"));
};

const one = (spec, provider) => {
  const dest = mkdtempSync(join(tmpdir(), `datasources-qs-${spec.name}-${spec.language}-${provider}-`));
  const isolated = isolate(provider, dest);
  const env = {
    PROVIDER: provider,
    DATASOURCE_URL: isolated.url,
    ...(process.env.MIGRATERS_PACKAGE ? { MIGRATERS_PACKAGE: process.env.MIGRATERS_PACKAGE } : {}),
  };
  console.log(`\n== ${spec.name} ${spec.language} ${provider} (${source}) ${dest}`);
  try {
    for (const line of spec["initialize-commands"] ?? []) run(rewrite(words(line), spec), dest, env);
    copySpec(spec, dest);
    for (const line of spec["run-commands"] ?? []) run(rewrite(words(line), spec), dest, env);
    for (const line of spec.build ?? []) run(rewrite(words(line), spec), dest, env);
    const actual = [];
    for (const line of spec.run ?? []) {
      const argv = rewrite(words(line), spec);
      const cmd = argv[0] === "dotnet" && argv[1] === "run" && !argv.includes("--no-build") ? [...argv, "--no-build"] : argv;
      actual.push(run(cmd, dest, env, "pipe").stdout ?? "");
    }
    const got = normalize(actual.join(""));
    const want = expectedOf(spec, provider);
    if (got !== want) throw new Error(`output mismatch\n--- expected\n${want}--- actual\n${got}`);
    console.log(`ok ${spec.name} ${provider}`);
  } finally {
    isolated.drop();
    if (keep) console.log(`KEEP ${dest}`);
    else rmSync(dest, { recursive: true, force: true });
  }
};

const specs = discover(here).map(load);
if (!specs.length) throw new Error("no quick_start.yaml found");
const failed = [];
for (const spec of specs) {
  for (const provider of spec.providers) {
    if (onlyProvider && provider !== onlyProvider) continue;
    try {
      one(spec, provider);
    } catch (error) {
      failed.push(`${spec.name}/${provider}: ${error.message}`);
      console.error(error.message);
    }
  }
}
if (failed.length) {
  console.error(`\n${failed.length} failed:\n${failed.join("\n")}`);
  process.exit(1);
}
