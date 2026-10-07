import { mkdtempSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { sqlPath, statements, withSeedVersion } from "./schema.ts";

describe("schema", () => {
  it("loads generated sql", () => {
    const sql = statements();
    expect(sql.some((item) => item.startsWith('CREATE TABLE "contacts"'))).toBe(true);
    expect(sql.some((item) => item.includes("INSERT INTO \"contact_sources\""))).toBe(true);
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
    expect(sqlPath()).toContain("0001_initial_up.sql");
    expect(() => sqlPath(mkdtempSync(join(tmpdir(), "contacts-")))).toThrow(/generated sql/);
  });
});
