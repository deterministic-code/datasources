import assert from "node:assert/strict";
import test from "node:test";
import { renderSkill, SKILL_ROOTS, zipSkill } from "./generate-skill.mjs";

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

test("skill is generated under skills/datasources", () => {
  assert.equal(SKILL_ROOTS[0], "skills/datasources");
});

test("zip contains datasources/SKILL.md", () => {
  const zip = zipSkill(files);
  assert.equal(zip.subarray(0, 2).toString(), "PK");
  assert.match(zip.toString("binary"), /datasources\/SKILL\.md/);
});
