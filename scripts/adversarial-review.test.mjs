import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import test from "node:test";
import { buildInput, feedbackPrompt, main, newBranches, parseSeen, stripFrontmatter, truncate } from "./adversarial-review.mjs";

const git = (cwd, ...args) => execFileSync("git", args, { cwd, encoding: "utf8" });

const repo = () => {
  const cwd = mkdtempSync(join(tmpdir(), "adversarial-"));
  git(cwd, "init", "-b", "main");
  git(cwd, "config", "user.email", "t@example.com");
  git(cwd, "config", "user.name", "t");
  mkdirSync(join(cwd, ".cursor/rules"), { recursive: true });
  writeFileSync(join(cwd, ".cursor/rules/adversarial-software-engineer.mdc"), "---\ndescription: x\n---\n\nBE HONEST\n");
  writeFileSync(join(cwd, "a.txt"), "one\n");
  git(cwd, "add", "-A");
  git(cwd, "commit", "-m", "init");
  return cwd;
};

const branch = (cwd, name, change = true) => {
  git(cwd, "checkout", "-b", name);
  if (change) {
    writeFileSync(join(cwd, `${name.replace("/", "_")}.txt`), "change\n");
    git(cwd, "add", "-A");
    git(cwd, "commit", "-m", name);
  }
  git(cwd, "checkout", "main");
};

const stub = "echo VERDICT: CHANGES REQUESTED";

test("stripFrontmatter removes the yaml header", () => {
  assert.equal(stripFrontmatter("---\ndescription: x\n---\n\nBody\n"), "Body");
});

test("newBranches drops the base and seen branches", () => {
  assert.deepEqual(newBranches(["main", "a", "b"], { a: "sha" }, "main"), ["b"]);
});

test("truncate keeps short diffs and marks long ones", () => {
  assert.equal(truncate("abc", 10), "abc");
  assert.match(truncate("abcdef", 3), /^abc\n\n\[diff truncated at 3 of 6 characters\]$/);
});

test("buildInput includes prompt, branch, and diff", () => {
  const input = buildInput("PROMPT", "feat/x", "+line");
  assert.ok(input.startsWith("PROMPT"));
  assert.match(input, /feat\/x/);
  assert.match(input, /\+line/);
});

test("feedbackPrompt wraps the review for the author", () => {
  assert.match(feedbackPrompt("feat/x", "FINDING\n"), /branch `feat\/x`[\s\S]*FINDING/);
});

test("parseSeen reads empty and populated state", () => {
  assert.deepEqual(parseSeen(""), {});
  assert.deepEqual(parseSeen('{"a":"sha"}'), { a: "sha" });
});

test("first run records a baseline and reviews nothing", () => {
  const cwd = repo();
  branch(cwd, "old");
  const logs = [];
  main(["--reviewer", stub], cwd, (line) => logs.push(line));
  assert.match(logs[0], /baseline: recorded 1/);
  assert.ok(!existsSync(join(cwd, "scripts/artifacts/adversarial")));
});

test("a new branch is reviewed once and then remembered", () => {
  const cwd = repo();
  branch(cwd, "old");
  main(["--reviewer", stub], cwd, () => {});
  branch(cwd, "feat/new");

  const first = [];
  main(["--reviewer", stub], cwd, (line) => first.push(line));
  assert.match(first[0], /reviewed feat\/new/);
  const out = readFileSync(join(cwd, "scripts/artifacts/adversarial/feat__new.md"), "utf8");
  assert.match(out, /VERDICT: CHANGES REQUESTED/);

  const second = [];
  main(["--reviewer", stub], cwd, (line) => second.push(line));
  assert.deepEqual(second, ["no new branches"]);
});

test("the reviewer receives the rule and the diff on stdin", () => {
  const cwd = repo();
  main([], cwd, () => {});
  branch(cwd, "feat/new");
  main(["--reviewer", "cat"], cwd, () => {});
  const out = readFileSync(join(cwd, "scripts/artifacts/adversarial/feat__new.md"), "utf8");
  assert.match(out, /BE HONEST/);
  assert.match(out, /\+change/);
  assert.doesNotMatch(out, /description: x/);
});

test("a branch with no changes is skipped and remembered", () => {
  const cwd = repo();
  main([], cwd, () => {});
  branch(cwd, "empty", false);
  const logs = [];
  main(["--reviewer", stub], cwd, (line) => logs.push(line));
  assert.match(logs[0], /skipped empty/);
  const again = [];
  main(["--reviewer", stub], cwd, (line) => again.push(line));
  assert.deepEqual(again, ["no new branches"]);
});

test("a failing reviewer leaves the branch unseen so it is retried", () => {
  const cwd = repo();
  main([], cwd, () => {});
  branch(cwd, "feat/new");
  const logs = [];
  main(["--reviewer", "exit 3"], cwd, (line) => logs.push(line));
  assert.match(logs[0], /failed feat\/new/);
  const retry = [];
  main(["--reviewer", stub], cwd, (line) => retry.push(line));
  assert.match(retry[0], /reviewed feat\/new/);
});

test("--dry-run lists new branches without reviewing", () => {
  const cwd = repo();
  main([], cwd, () => {});
  branch(cwd, "feat/new");
  const logs = [];
  main(["--dry-run"], cwd, (line) => logs.push(line));
  assert.deepEqual(logs, ["feat/new"]);
  assert.ok(!existsSync(join(cwd, "scripts/artifacts/adversarial")));
});

test("--force reviews an already seen branch again", () => {
  const cwd = repo();
  main([], cwd, () => {});
  branch(cwd, "feat/new");
  main(["--reviewer", stub], cwd, () => {});
  const logs = [];
  main(["--reviewer", stub, "--force", "feat/new"], cwd, (line) => logs.push(line));
  assert.match(logs[0], /reviewed feat\/new/);
});

test("--all reviews existing branches on the first run", () => {
  const cwd = repo();
  branch(cwd, "old");
  const logs = [];
  main(["--all", "--reviewer", stub], cwd, (line) => logs.push(line));
  assert.match(logs[0], /reviewed old/);
});

test("an unknown argument is rejected", () => {
  assert.throws(() => main(["--nope"], repo(), () => {}), /unknown argument --nope/);
});

test("--base and --max-diff-bytes are honored", () => {
  const cwd = repo();
  main([], cwd, () => {});
  branch(cwd, "feat/new");
  main(["--base", "main", "--max-diff-bytes", "20", "--reviewer", "cat"], cwd, () => {});
  assert.match(readFileSync(join(cwd, "scripts/artifacts/adversarial/feat__new.md"), "utf8"), /diff truncated at 20/);
});

test("--dry-run --branch prints the diff and input without a model, state, or output", () => {
  const cwd = repo();
  branch(cwd, "feat/new");
  const logs = [];
  main(["--dry-run", "--branch", "feat/new", "--reviewer", "exit 9"], cwd, (line) => logs.push(line));
  assert.match(logs[0], /1 file changed/);
  assert.match(logs[1], /BE HONEST[\s\S]*\+change/);
  assert.ok(!existsSync(join(cwd, ".git/adversarial-reviewed.json")));
  assert.ok(!existsSync(join(cwd, "scripts/artifacts/adversarial")));
});

test("--branch reviews a merged branch when --base points at its parent", () => {
  const cwd = repo();
  branch(cwd, "feat/new");
  git(cwd, "merge", "--no-ff", "-m", "merge", "feat/new");
  const empty = [];
  main(["--dry-run", "--branch", "feat/new"], cwd, (line) => empty.push(line));
  assert.match(empty[0], /no changes in feat\/new/);
  const logs = [];
  main(["--branch", "feat/new", "--base", "feat/new~1", "--reviewer", stub], cwd, (line) => logs.push(line));
  assert.match(logs[0], /VERDICT: CHANGES REQUESTED/);
  assert.ok(!existsSync(join(cwd, ".git/adversarial-reviewed.json")));
});

test("--branch rejects an unknown ref", () => {
  assert.throws(() => main(["--branch", "nope"], repo(), () => {}), /unknown ref nope/);
});
