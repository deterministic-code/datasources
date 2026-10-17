import { mkdtempSync, mkdirSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { sqlPath, statements, upSqlFrom, withSeedVersion } from "./schema.ts";

describe("schema", () => {
  it("loads generated sql", () => {
    const sql = statements();
    expect(sql.some((item) => item.startsWith('CREATE TABLE "contacts"'))).toBe(true);
    expect(sql.some((item) => item.startsWith('CREATE TABLE "contact_settings"'))).toBe(true);
    expect(sql.some((item) => item.startsWith('CREATE TABLE "contact_preferences"'))).toBe(true);
    expect(sql.every((item) => !item.includes('CREATE TABLE "bases"'))).toBe(true);
    expect(sql.some((item) => item.includes("INSERT INTO \"contact_sources\""))).toBe(true);
    expect(sql.some((item) => item.includes("DROP COLUMN"))).toBe(true);
    expect(sql.some((item) => item.includes("DROP TABLE"))).toBe(true);
    expect(sql.every((item) => item.endsWith(";"))).toBe(true);
  });

  it("splits comments triggers and seeds", () => {
    expect(
      statements(`-- heading
CREATE TABLE "t" ("id" INTEGER);
CREATE TRIGGER "trg"
AFTER UPDATE ON "t"
BEGIN
  UPDATE "t" SET "id" = "id" WHERE "id" = OLD."id";
END;
INSERT INTO "t" ("id") VALUES (1);
`),
    ).toEqual([
      'CREATE TABLE "t" ("id" INTEGER);',
      `CREATE TRIGGER "trg"
AFTER UPDATE ON "t"
BEGIN
  UPDATE "t" SET "id" = "id" WHERE "id" = OLD."id";
END;`,
      'INSERT INTO "t" ("id", "version") VALUES (1, X\'00\');',
    ]);
  });

  it("covers seed and word edges", () => {
    expect(withSeedVersion("SELECT 1;")).toBe("SELECT 1;");
    expect(withSeedVersion('INSERT INTO "t" ("id", "version") VALUES (1, X\'00\');')).toBe(
      'INSERT INTO "t" ("id", "version") VALUES (1, X\'00\');',
    );
    expect(withSeedVersion("INSERT INTO \"t\" id VALUES 1;")).toBe('INSERT INTO "t" id VALUES 1;');
    expect(withSeedVersion("INSERT INTO `t` (`id`) VALUES (1);")).toBe(
      "INSERT INTO `t` (`id`, `version`) VALUES (1, X'00');",
    );
    expect(
      statements(`CREATE OR REPLACE FUNCTION set_updated_at() RETURNS TRIGGER AS $$
BEGIN
  NEW."updated" = NOW() AT TIME ZONE 'UTC';
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE TABLE "t" ("id" INTEGER);
`),
    ).toEqual([
      `CREATE OR REPLACE FUNCTION set_updated_at() RETURNS TRIGGER AS $$
BEGIN
  NEW."updated" = NOW() AT TIME ZONE 'UTC';
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;`,
      'CREATE TABLE "t" ("id" INTEGER);',
    ]);
    expect(statements("BEGIN")).toEqual([]);
    expect(statements(";")).toEqual([";"]);
    expect(statements("END;")).toEqual(["END;"]);
    expect(statements("XEND;")).toEqual(["XEND;"]);
    expect(statements("BEGINNING;").length).toBe(1);
    expect(statements("SELECT 1; \n;")).toEqual(["SELECT 1;", ";"]);
    expect(statements("END ;")).toEqual(["END ;"]);
    expect(
      statements(`BEGIN
  UPDATE "t" SET "id" = 1;
END;
CREATE TABLE "XBEGIN" ("id" INTEGER);
`).length,
    ).toBe(2);
    expect(sqlPath()).toMatch(/migrations$/);
    expect(() => sqlPath(mkdtempSync(join(tmpdir(), "contacts-")))).toThrow(/generated sql/);
    const missing = join(tmpdir(), `contacts-missing-${Date.now()}`);
    expect(() => upSqlFrom(missing)).toThrow(/generated sql/);
    const file = join(mkdtempSync(join(tmpdir(), "contacts-file-")), "one.sql");
    writeFileSync(file, "CREATE TABLE \"t\" (\"id\" INTEGER);\n");
    expect(upSqlFrom(file)).toBe('CREATE TABLE "t" ("id" INTEGER);\n');
    const initialDir = mkdtempSync(join(tmpdir(), "contacts-initial-"));
    writeFileSync(join(initialDir, "0001_initial_up.sql"), "CREATE TABLE \"i\" (\"id\" INTEGER);\n");
    expect(upSqlFrom(initialDir)).toBe('CREATE TABLE "i" ("id" INTEGER);\n');
    expect(() => upSqlFrom(mkdtempSync(join(tmpdir(), "contacts-empty-")))).toThrow(/generated sql/);
    const nested = mkdtempSync(join(tmpdir(), "contacts-nested-"));
    mkdirSync(join(nested, "samples", "typescript", "contacts-crud-enriched-medium-rest", "generated", "sql", "sqlite", "migrations"), { recursive: true });
    writeFileSync(
      join(nested, "samples", "typescript", "contacts-crud-enriched-medium-rest", "generated", "sql", "sqlite", "migrations", "0001_initial_up.sql"),
      "CREATE TABLE \"n\" (\"id\" INTEGER);\n",
    );
    expect(sqlPath(nested)).toMatch(/migrations$/);
  });
});
