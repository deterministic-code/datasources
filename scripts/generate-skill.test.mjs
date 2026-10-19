import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import test from "node:test";
import { fileURLToPath } from "node:url";
import { renderSkill, skillRouter, SKILL_REPO } from "./generate-skill.mjs";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const slug = (title) => title.toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/^-|-$/g, "");

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
  assert.match(files["downloading.md"], /@deterministic-code\/datasources-sqlite/);
  assert.match(files["downloading.md"], /datasources-sqlite/);
  assert.match(files["downloading.md"], /Datasources\.Sqlite/);
});

test("repo-root SKILL.md links into skills/", () => {
  const router = skillRouter("skills/");
  assert.match(router, /\]\(skills\/setup\.md\)/);
  assert.equal(SKILL_REPO, "deterministic-code/datasources");
});

test("skill query headings match evals.json", () => {
  const page = readFileSync(join(root, "docs/skill.md"), "utf8");
  const headings = [...page.matchAll(/^## (.+)$/gm)].map((match) =>
    slug(match[1].replace(/^\d+\.\s+/, "")),
  );
  const evals = JSON.parse(readFileSync(join(root, "docs/evals/evals.json"), "utf8")).evals;
  const ids = evals.map((item) => item.id);
  for (const heading of headings) assert.ok(ids.includes(heading), `missing eval ${heading}`);
  assert.equal(ids.length, headings.length);
});

test("generated skill omits example queries", () => {
  assert.equal(files["skill-queries.md"], undefined);
  assert.doesNotMatch(files["SKILL.md"], /skill-queries/);
});
