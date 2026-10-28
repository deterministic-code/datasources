import { spawnSync } from "node:child_process";
import { createRequire } from "node:module";
import {
  existsSync,
  mkdtempSync,
  readdirSync,
  readFileSync,
  rmSync,
  statSync,
} from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));
const repo = dirname(here);
const { parse } = createRequire(join(repo, "cli/package.json"))("yaml");
const keep = process.env.KEEP === "1";
const only = flag("provider");
const composeProject = "datasources-qs";
const runners = {
  csharp: ["dotnet", "run"],
  rust: ["cargo", "run"],
  typescript: ["npx", "tsx"],
};

const startsWith = (argv, prefix) =>
  prefix.length > 0 && prefix.every((word, i) => argv[i] === word);

function flag(name) {
  const i = process.argv.indexOf(`--${name}`);
  return i >= 0 ? process.argv[i + 1] : undefined;
}

const fail = (label, result) => {
  const text = [result.stderr, result.stdout].filter(Boolean).join("\n").trim();
  throw new Error(`${label}${text ? `\n${text}` : `\nexit ${result.status}`}`);
};

const run = (argv, cwd, env, io = "inherit") => {
  const [command, ...args] = argv;
  const result = spawnSync(command, args, {
    cwd,
    encoding: "utf8",
    stdio: io === "pipe" ? ["ignore", "pipe", "pipe"] : "inherit",
    env,
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
  if (!spec?.name || !spec.language || !spec.suites?.length)
    throw new Error(`${path}: name, language, suites required`);
  return { path, dir: dirname(path), ...spec };
};

const exportLine = (line) => {
  const match = line.trim().match(/^export\s+([A-Za-z_][A-Za-z0-9_]*)=(.*)$/);
  if (!match) return undefined;
  const value = match[2];
  return {
    key: match[1],
    value: value.startsWith('"') && value.endsWith('"') ? value.slice(1, -1) : value,
  };
};

const normalize = (text) => text.replace(/\r\n/g, "\n").replace(/\s+$/, "") + "\n";

const expectedOf = (spec, suite) => {
  const path = join(spec.dir, suite.expected || spec.expected || "expected.txt");
  if (!existsSync(path)) throw new Error(`${spec.path}: missing ${path}`);
  return normalize(readFileSync(path, "utf8"));
};

const composeArgv = (argv) =>
  argv[0] === "docker" && argv[1] === "compose"
    ? ["docker", "compose", "-p", composeProject, ...argv.slice(2)]
    : argv;

const composeDown = (cwd, env) => {
  spawnSync("docker", ["compose", "-p", composeProject, "down", "-v", "--remove-orphans"], {
    cwd,
    encoding: "utf8",
    env,
    stdio: "inherit",
  });
};

const one = (spec, suite) => {
  const dest = mkdtempSync(join(repo, "datasources-qs-"));
  console.log(`\n== ${spec.name} ${spec.language} ${suite.name} ${dest}`);
  const env = { ...process.env };
  const actual = [];
  let compose = false;
  try {
    for (const line of suite.commands ?? []) {
      const exported = exportLine(line);
      if (exported) {
        env[exported.key] = exported.value;
        continue;
      }
      const raw = words(line);
      const argv = composeArgv(raw);
      if (argv !== raw) {
        compose = true;
        if (argv.includes("up")) composeDown(dest, env);
      }
      const capture = startsWith(argv, runners[spec.language] ?? []);
      const result = run(argv, dest, env, capture ? "pipe" : "inherit");
      if (capture) actual.push(result.stdout ?? "");
    }
    const want = expectedOf(spec, suite);
    if (actual.length === 0) throw new Error(`${spec.name} ${suite.name}: no captured run output`);
    for (const stdout of actual) {
      const got = normalize(stdout);
      if (got !== want) throw new Error(`output mismatch\n--- expected\n${want}--- actual\n${got}`);
    }
    console.log(`ok ${spec.name} ${suite.name}`);
  } finally {
    if (compose) composeDown(dest, env);
    if (keep) console.log(`KEEP ${dest}`);
    else rmSync(dest, { recursive: true, force: true });
  }
};

const specs = discover(here).map(load);
if (!specs.length) throw new Error("no quick_start.yaml found");
for (const spec of specs) {
  for (const suite of spec.suites) {
    if (only && suite.name !== only && !(suite.providers ?? []).includes(only)) continue;
    one(spec, suite);
  }
}
