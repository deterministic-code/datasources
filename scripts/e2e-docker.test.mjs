import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { chmodSync, mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const scripts = dirname(fileURLToPath(import.meta.url));
const driver = join(scripts, "e2e-docker.sh");

const runDriver = (args, { sleep = "0", fail = "", diff = "" } = {}) => {
  const home = mkdtempSync(join(tmpdir(), "e2e-docker-"));
  const bin = join(home, "bin");
  const log = join(home, "docker.log");
  const lock = join(home, "docker.lock");
  mkdirSync(bin);
  writeFileSync(
    join(bin, "docker"),
    `#!/bin/sh
lock() { while ! mkdir "$DOCKER_LOCK" 2>/dev/null; do sleep 0.01; done; }
unlock() { rmdir "$DOCKER_LOCK"; }
lock
printf 'CMD %s\\n' "$*" >> "$DOCKER_LOG"
unlock
case " $* " in
  *" sqlite.sh "*)
    lock
    printf 'START %s\\n' "$COMPOSE_PROJECT_NAME" >> "$DOCKER_LOG"
    unlock
    sleep "$FAKE_DOCKER_SLEEP"
    if [ "$COMPOSE_PROJECT_NAME" = "$FAKE_DOCKER_FAIL" ]; then
      exit 1
    fi
    if [ "$COMPOSE_PROJECT_NAME" = "$FAKE_DOCKER_DIFF" ]; then
      printf '%s\\n' '--- expected.txt' '+++ -' '-created 1 Ada' '+created 3 Ada'
      exit 1
    fi
    lock
    printf 'END %s\\n' "$COMPOSE_PROJECT_NAME" >> "$DOCKER_LOG"
    unlock
    ;;
esac
exit 0
`,
  );
  chmodSync(join(bin, "docker"), 0o755);
  return {
    log,
    result: spawnSync("bash", [driver, ...args], {
      encoding: "utf8",
      timeout: 60_000,
      env: {
        ...process.env,
        HOME: home,
        PATH: `${bin}:${process.env.PATH}`,
        DOCKER_LOG: log,
        DOCKER_LOCK: lock,
        FAKE_DOCKER_SLEEP: sleep,
        FAKE_DOCKER_FAIL: fail,
        FAKE_DOCKER_DIFF: diff,
      },
    }),
  };
};

const overlap = (log) => {
  const active = new Set();
  let max = 0;
  let rust = 0;
  let maxRust = 0;
  for (const line of readFileSync(log, "utf8").split("\n")) {
    const start = line.startsWith("START ") ? line.slice(6) : "";
    const end = line.startsWith("END ") ? line.slice(4) : "";
    const name = start || end;
    if (!name) continue;
    const isRust = name.includes("-rust-") || name.endsWith("-rust");
    if (start) {
      active.add(name);
      if (isRust) rust += 1;
    } else {
      active.delete(name);
      if (isRust) rust -= 1;
    }
    max = Math.max(max, active.size);
    maxRust = Math.max(maxRust, rust);
  }
  return { max, maxRust };
};

test("samples share a slot cap, rust stays single, and both scripts run in one container", () => {
  const { log, result } = runDriver([], { sleep: "0.6" });
  assert.equal(result.status, 0, result.stderr || result.stdout);
  const text = readFileSync(log, "utf8");
  const runs = text.split("\n").filter((line) => line.includes("sqlite.sh && sh mysql-postgres.sh"));
  assert.equal(runs.length, 15);
  assert.equal(text.includes("e2e-node.Dockerfile"), true);
  assert.equal(text.includes("e2e-dotnet.Dockerfile"), true);
  assert.equal(text.includes("e2e-rust.Dockerfile"), true);
  const { max, maxRust } = overlap(log);
  assert.equal(maxRust, 1);
  assert.ok(max >= 2 && max <= 3, `max ${max}`);
});

test("--local shares a slot cap, rust stays single, and both scripts run in one container", () => {
  const { log, result } = runDriver(["--local"], { sleep: "0.6" });
  assert.equal(result.status, 0, result.stderr || result.stdout);
  const text = readFileSync(log, "utf8");
  const runs = text.split("\n").filter((line) => line.includes("sqlite.sh --local && sh mysql-postgres.sh --local"));
  assert.equal(runs.length, 15);
  const { max, maxRust } = overlap(log);
  assert.equal(maxRust, 1);
  assert.ok(max >= 2 && max <= 3, `max ${max}`);
});

test("--local prepares the CLI once and mounts the checkout read-only", () => {
  const { log, result } = runDriver(["--local", "samples/typescript/contacts-simple-cli"]);
  assert.equal(result.status, 0, result.stderr || result.stdout);
  const text = readFileSync(log, "utf8");
  const prepares = text.split("\n").filter((line) => line.includes("npm run prepare --prefix"));
  assert.equal(prepares.length, 1);
  assert.match(
    prepares[0],
    /npm install --prefix \/datasources\/cli && npm run prepare --prefix \/datasources\/cli && npm install --prefix \/datasources\/typescript && npm run build --prefix \/datasources\/typescript/,
  );
  const runs = text.split("\n").filter((line) => line.includes("compose run"));
  assert.equal(runs.length, 1);
  assert.match(runs[0], /:\/datasources:ro/);
  assert.match(runs[0], /\/cli:\/datasources\/cli:ro/);
  assert.match(runs[0], /\/typescript:\/datasources\/typescript:ro/);
  assert.match(runs[0], /\/csharp:\/datasources\/csharp:ro/);
  assert.match(runs[0], /DATASOURCES_BUILD_ROOT=\/tmp\/datasources-build/);
  assert.match(runs[0], /DATASOURCES_TS_PREBUILT=1/);
  assert.equal(runs[0].includes("npm install --prefix"), false);
  assert.equal(runs[0].includes("cp -R"), false);
  assert.match(runs[0], /sqlite\.sh --local && sh mysql-postgres\.sh --local/);
});

test("contacts CLI quick starts set mysql_native_password like samples", () => {
  const root = join(scripts, "..");
  const expected = readFileSync(join(root, "scripts/mysql/init/02-native-password.sql"), "utf8");
  for (const language of ["typescript", "csharp", "rust"]) {
    const path = join(root, "quick_starts/contacts_cli", language, "mysql/init/02-native-password.sql");
    assert.equal(readFileSync(path, "utf8"), expected, path);
  }
});

test("csharp compose isolates global packages per project", () => {
  const root = join(scripts, "..");
  const files = [
    "samples/csharp/contacts-simple-cli/compose.yml",
    "samples/csharp/contacts-crud-enriched-medium-rest/compose.yml",
    "samples/csharp/complex-game-rest/compose.yml",
    "samples/csharp/user-roles-simple-cli/compose.yml",
    "samples/csharp/outliers/compose.yml",
    "quick_starts/contacts_cli/csharp/compose.yml",
  ];
  for (const file of files) {
    const text = readFileSync(join(root, file), "utf8");
    assert.match(text, /NUGET_PACKAGES: \/nuget\/packages\/\$\{COMPOSE_PROJECT_NAME/);
    assert.match(text, /NUGET_HTTP_CACHE_PATH: \/nuget\/http-cache\/\$\{COMPOSE_PROJECT_NAME/);
    assert.equal(text.includes("NUGET_PACKAGES: /nuget\n"), false, file);
    assert.equal(text.includes("NUGET_HTTP_CACHE_PATH: /nuget/http-cache\n"), false, file);
  }
});

test("a failing sample exits non-zero", () => {
  const { result } = runDriver(["typescript"], {
    fail: "e2e-samples-typescript-contacts-simple-cli",
  });
  assert.notEqual(result.status, 0);
  assert.equal(result.error, undefined);
});

test("an expected.txt diff exits non-zero", () => {
  const { result } = runDriver(["typescript"], {
    diff: "e2e-samples-typescript-contacts-simple-cli",
  });
  assert.notEqual(result.status, 0);
  assert.equal(result.error, undefined);
});
