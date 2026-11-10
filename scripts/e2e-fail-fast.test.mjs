import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { dirname, join } from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";

const scripts = dirname(fileURLToPath(import.meta.url));
const helper = join(scripts, "e2e-fail-fast.sh");

const sh = (body) => spawnSync("sh", ["-c", `. "${helper}"; ${body}`], { encoding: "utf8" });

test("watcher exits on the first warning line and does not read the rest", () => {
  const result = spawnSync(
    "sh",
    [
      "-c",
      `. "${helper}"; { echo ok; echo 'warning: first'; sleep 1; echo later; } | e2e_watch`,
    ],
    { encoding: "utf8" },
  );
  assert.notEqual(result.status, 0);
  assert.match(result.stdout, /warning: first/);
  assert.doesNotMatch(result.stdout, /later/);
  assert.doesNotMatch(result.stdout, /AFTER/);
});

test("0 Warning(s), 0 Error(s), npm env-config warn, npm deprecated, git integrity skip, npm warn exec, NU1902, and cargo future-incompat do not fail", () => {
  const result = spawnSync(
    "sh",
    [
      "-c",
      `. "${helper}"; printf '%s\\n' '    0 Warning(s)' '    0 Error(s)' 'npm warn Unknown env config "devdir"' 'npm warn deprecated glob@10.5.0: Old versions of glob are not supported' 'npm warn skipping integrity check for git dependency ssh://git@github.com/deterministic-code/generators-common.git' 'npm warn exec The following package was not found and will be installed: @deterministic-code/datasources-cli@0.1.56' "MigrateMysql.csproj : warning NU1902: Package 'MySqlConnector' 2.4.0 has a known moderate severity vulnerability" 'warning: the following packages contain code that will be rejected by a future version of Rust: proc-macro-error2 v2.0.1' | e2e_watch`,
    ],
    { encoding: "utf8" },
  );
  assert.equal(result.status, 0);
});

test("rustc error and npm error fail", () => {
  for (const line of [
    "error[E0061]: this method takes 1 argument but 2 arguments were supplied",
    "error: could not compile `contacts-simple-cli`",
    "npm error code ERESOLVE",
  ]) {
    const result = sh(`e2e_fail_line ${JSON.stringify(line)}`);
    assert.equal(result.status, 0, line);
  }
});

test("false | true under pipefail exits non-zero", () => {
  const result = spawnSync("bash", ["-e", "-o", "pipefail", "-c", "false | true"], { encoding: "utf8" });
  assert.notEqual(result.status, 0);
});

test("a ||-called helper returns the inner non-zero status", () => {
  const result = spawnSync(
    "sh",
    [
      "-c",
      `
run_compose() {
  (
    run=0
    false || run=$?
    if [ "$run" -ne 0 ]; then
      return "$run"
    fi
    printf 'done\\n'
    return 0
  )
}
status=0
run_compose || status=$?
exit "$status"
`,
    ],
    { encoding: "utf8" },
  );
  assert.notEqual(result.status, 0);
});
