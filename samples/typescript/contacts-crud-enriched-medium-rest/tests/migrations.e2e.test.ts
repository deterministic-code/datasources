import { execFileSync } from "node:child_process";
import { mkdtempSync, readdirSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { basename, dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import Database from "better-sqlite3";
import { parse } from "yaml";
import { describe, expect, it } from "vitest";
import { statements } from "../src/schema.ts";
import {
  applyDocument,
  snapshotsEqual,
  type Json,
} from "../../../../typescript/code-generators/src/sql/common/version-apply.ts";

const sample = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const repo = resolve(sample, "../../..");
const spec = resolve(sample, "../../contacts-crud-enriched-medium-rest-shared/deterministic");
const ups = (dir: string) =>
  readdirSync(dir)
    .filter((name) => /^\d{4}_up\.sql$/.test(name))
    .sort()
    .map((name) => join(dir, name));

const tables = (db: Database.Database) =>
  db.prepare("SELECT name FROM sqlite_master WHERE type = 'table' ORDER BY name").all() as { name: string }[];
const columns = (db: Database.Database, table: string) =>
  (db.prepare(`PRAGMA table_info("${table}")`).all() as { name: string }[]).map((row) => row.name);

describe("versioned migrations", () => {
  it("replays frozen diffs to live types.yaml", () => {
    let snapshot: { [path: string]: Json } = {};
    for (const name of readdirSync(spec)
      .filter((name) => /^version-\d{8}T\d{6}Z-\d{3}$/.test(name))
      .sort()) {
      snapshot = applyDocument(
        snapshot,
        parse(readFileSync(join(spec, name, "diff.yaml"), "utf8")) as { [path: string]: Json },
      );
    }
    expect(snapshotsEqual(snapshot["types.yaml"], parse(readFileSync(join(spec, "types.yaml"), "utf8")) as Json)).toBe(
      true,
    );
  });

  it("drops extra fields and tables across three versions", () => {
    const out = mkdtempSync(join(tmpdir(), "contacts-mig-"));
    try {
      execFileSync(
        process.execPath,
        [
          "--import",
          "tsx",
          join(repo, "cli/src/main.ts"),
          "migrations",
          "create",
          "--input",
          spec,
          "--migrations-root",
          join(out, "sql"),
        ],
        { encoding: "utf8", cwd: join(repo, "cli") },
      );
      const files = ups(join(out, "sql/sqlite/migrations"));
      expect(files.map((file) => basename(file))).toEqual(["0001_up.sql", "0002_up.sql", "0003_up.sql"]);
      const db = new Database(":memory:");
      db.exec("PRAGMA foreign_keys = ON");
      for (const sql of statements(readFileSync(files[0]!, "utf8"))) db.exec(sql);
      expect(tables(db).map((row) => row.name)).toEqual(expect.arrayContaining(["contacts", "tags", "note_pads"]));
      expect(columns(db, "contacts")).toContain("middle_name");
      expect(columns(db, "addresses")).toContain("line3");
      for (const sql of statements(readFileSync(files[1]!, "utf8"))) db.exec(sql);
      expect(tables(db).map((row) => row.name)).not.toContain("tags");
      expect(columns(db, "contacts")).not.toContain("middle_name");
      expect(columns(db, "addresses")).toContain("line3");
      for (const sql of statements(readFileSync(files[2]!, "utf8"))) db.exec(sql);
      expect(tables(db).map((row) => row.name)).not.toContain("note_pads");
      expect(columns(db, "addresses")).not.toContain("line3");
      expect(tables(db).map((row) => row.name)).toEqual(expect.arrayContaining(["contacts"]));
    } finally {
      rmSync(out, { recursive: true, force: true });
    }
  });
});
