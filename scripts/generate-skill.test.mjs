import assert from "node:assert/strict";
import test from "node:test";
import { renderSkill, skillRouter, SKILL_REPO } from "./generate-skill.mjs";

const files = renderSkill();

test("SKILL.md is under 500 lines", () => {
  assert.ok(files["SKILL.md"].split("\n").length < 500);
});

test("SKILL.md links exist", () => {
  for (const match of files["SKILL.md"].matchAll(/\]\(([^)]+)\)/g)) {
    const file = match[1].replace(/#.*$/, "");
    if (!file || file.startsWith("http")) continue;
    assert.ok(files[file], `missing ${file}`);
  }
});

test("topic files expand examples", () => {
  for (const [name, body] of Object.entries(files)) {
    if (name === "SKILL.md") continue;
    assert.doesNotMatch(body, /::: example/);
  }
  assert.match(files["command.md"], /```(?:csharp|typescript|rust)/);
});

test("downloading lists sqlite packages", () => {
  assert.match(files["downloading.md"], /@datasources\/sqlite/);
  assert.match(files["downloading.md"], /datasources-sqlite/);
  assert.match(files["downloading.md"], /Datasources\.Sqlite/);
});

test("repo-root SKILL.md links into skills/", () => {
  const root = skillRouter("skills/");
  assert.match(root, /\]\(skills\/setup\.md\)/);
  assert.equal(SKILL_REPO, "deterministic-code/datasources");
});
