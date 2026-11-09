import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { chmodSync, mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const scripts = dirname(fileURLToPath(import.meta.url));
const driver = join(scripts, "e2e-docker.sh");

const runDriver = (args, { sleep = "0", fail = "" } = {}) => {
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
    result: spawnSync("sh", [driver, ...args], {
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
  assert.equal(runs.length, 12);
  assert.equal(text.includes("e2e-node.Dockerfile"), true);
  assert.equal(text.includes("e2e-dotnet.Dockerfile"), true);
  assert.equal(text.includes("e2e-rust.Dockerfile"), true);
  const { max, maxRust } = overlap(log);
  assert.equal(maxRust, 1);
  assert.ok(max >= 2 && max <= 3, `max ${max}`);
});

test("--local stays serial so CLI installs do not share one checkout", () => {
  const { log, result } = runDriver(["--local", "typescript"], { sleep: "0.4" });
  assert.equal(result.status, 0, result.stderr || result.stdout);
  const runs = readFileSync(log, "utf8")
    .split("\n")
    .filter((line) => line.includes("sqlite.sh --local && sh mysql-postgres.sh --local"));
  assert.equal(runs.length, 4);
  assert.equal(overlap(log).max, 1);
});

test("--local installs the CLI once per sample and still runs both scripts", () => {
  const { log, result } = runDriver(["--local", "samples/typescript/contacts-simple-cli"]);
  assert.equal(result.status, 0, result.stderr || result.stdout);
  const runs = readFileSync(log, "utf8")
    .split("\n")
    .filter((line) => line.includes("compose run"));
  assert.equal(runs.length, 1);
  assert.match(runs[0], /\/datasources\/cli\/node_modules/);
  assert.match(runs[0], /npm run prepare --prefix/);
  assert.match(runs[0], /sqlite\.sh --local && sh mysql-postgres\.sh --local/);
});

test("a failing sample exits non-zero", () => {
  const { result } = runDriver(["typescript"], {
    fail: "e2e-samples-typescript-contacts-simple-cli",
  });
  assert.notEqual(result.status, 0);
  assert.equal(result.error, undefined);
});
