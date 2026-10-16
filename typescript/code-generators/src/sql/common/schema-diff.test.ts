import assert from "node:assert/strict";
import { describe, it } from "vitest";
import type { TypeField } from "@deterministic-code/deterministic-generators-common/parser";
import { createCasing } from "./default-casing.ts";
import { SqlMapping } from "./sql-mapping.ts";
import { buildLiveTables, type LiveTable, type SqlTable } from "./sql-schema.ts";
import { renderSql } from "./sql-dialect.ts";
import { schemaDiff } from "./schema-diff.ts";

const field = (name: string, extra: Partial<TypeField> = {}): TypeField => ({
  name,
  type: extra.type ?? (name === "id" ? "integer" : "string"),
  kind: "primitive",
  base: extra.type ?? (name === "id" ? "integer" : "string"),
  isArray: false,
  isNullable: extra.isNullable ?? false,
  ...extra,
});

const table = (name: string, fields: TypeField[], extra: Partial<SqlTable> = {}): SqlTable => ({
  name,
  tags: ["datasource_type"],
  kind: "inherit",
  inherits: "set",
  fields,
  indexes: extra.indexes ?? [],
  uniqueIndexFields: extra.uniqueIndexFields ?? [],
  overlays: extra.overlays ?? [],
  ...(extra.mapping !== undefined ? { mapping: extra.mapping } : {}),
});

const snap = (tables: SqlTable[]) => {
  const mapping = new SqlMapping(createCasing({}), tables);
  return { tables: buildLiveTables(tables, mapping), mapping };
};

const sql = (before: SqlTable[], after: SqlTable[]) =>
  schemaDiff("sqlite", snap(before), snap(after)).map((statement) => renderSql("sqlite", statement));

describe("schemaDiff", () => {
  it("creates an added table and drops a removed table", () => {
    const users = table("user", [field("id", { isId: true }), field("email")]);
    assert.equal(sql([], [users]).some((item) => item.startsWith("CREATE TABLE")), true);
    assert.deepEqual(sql([users], []), ['DROP TABLE IF EXISTS "users";']);
  });

  it("adds, drops, and renames columns", () => {
    const before = table("user", [field("id", { isId: true }), field("email")]);
    const added = table("user", [field("id", { isId: true }), field("email"), field("name")]);
    assert.equal(
      sql([before], [added]).some((item) => item.includes("ADD COLUMN")),
      true,
    );
    const dropped = table("user", [field("id", { isId: true })]);
    assert.equal(
      sql([before], [dropped]).some((item) => item.includes("DROP COLUMN")),
      true,
    );
    const renamed = table(
      "user",
      [field("id", { isId: true }), field("email")],
      { overlays: [{ name: "email", mapping: "email_address" }] },
    );
    assert.equal(
      sql([before], [renamed]).some((item) => item.includes("RENAME COLUMN")),
      true,
    );
  });

  it("throws when the primary key changes", () => {
    const before = table("user", [field("id", { isId: true }), field("email")]);
    const after = table("user", [field("email", { isId: true }), field("id")]);
    assert.throws(() => sql([before], [after]), /primary key or identity/);
  });
});
