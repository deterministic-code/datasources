import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));

const binDir = () =>
  [
    join(here, "../samples/typescript/user-roles-simple-cli/node_modules/.bin"),
    join(here, "../samples/typescript/contacts-crud-enriched-medium-rest/node_modules/.bin"),
    join(process.cwd(), "node_modules/.bin"),
  ].find((dir) => existsSync(join(dir, "migrate-up")));

const sqliteConnection = (connection) => {
  if (!connection?.startsWith("sqlite:") || connection.includes(":memory:")) return connection;
  const rest = connection.slice("sqlite:".length);
  const file = rest.startsWith("////") ? rest.slice(3) : rest.replace(/^\/\/\//, "");
  if (!existsSync(file)) {
    mkdirSync(dirname(file), { recursive: true });
    writeFileSync(file, "");
  }
  return file;
};

const run = (command, { provider, connection, migrationsRoot, cwd, extra = [], root = true }) => {
  const bins = binDir();
  const args = [
    "--provider",
    provider,
    "--connection",
    provider === "sqlite" ? sqliteConnection(connection) : connection,
    ...(root ? ["--migrations-root", migrationsRoot] : []),
    ...extra,
  ];
  return spawnSync(bins ? join(bins, command) : "npx", bins ? args : ["--yes", "--package", "@deterministic-code/migraters", command, ...args], {
    encoding: "utf8",
    cwd: bins ? dirname(dirname(bins)) : (cwd ?? process.cwd()),
    maxBuffer: 20 * 1024 * 1024,
  });
};

const requireOk = (result, label) => {
  if ((result.status ?? 1) !== 0)
    throw new Error(`${label}: ${result.stderr || result.stdout || "failed"}`);
  return `${result.stdout}${result.stderr}`;
};

export const migrateUp = (args) => {
  requireOk(run("migrate-setup", { ...args, root: false }), "migrate-setup");
  return requireOk(run("migrate-up", args), "migrate-up");
};

export const migrateDown = (args) =>
  requireOk(run("migrate-down", { ...args, extra: ["--confirm"] }), "migrate-down");

const flag = (argv, name) => {
  const i = argv.indexOf(name);
  return i >= 0 ? argv[i + 1] : undefined;
};

if (process.argv[1] && fileURLToPath(import.meta.url) === process.argv[1]) {
  const argv = process.argv.slice(2);
  const args = {
    provider: flag(argv, "--provider"),
    connection: flag(argv, "--connection"),
    migrationsRoot: flag(argv, "--migrations-root"),
    cwd: flag(argv, "--cwd"),
  };
  try {
    if (argv[0] === "up") process.stdout.write(migrateUp(args));
    else if (argv[0] === "down") process.stdout.write(migrateDown(args));
    else throw new Error(`Usage: node ${here}/migraters.mjs up|down --provider <name> --connection <url> --migrations-root <dir>`);
  } catch (error) {
    process.stderr.write(`${error instanceof Error ? error.message : error}\n`);
    process.exit(1);
  }
}
