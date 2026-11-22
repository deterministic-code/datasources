import assert from "node:assert/strict";
import { describe, it } from "vitest";
import type { TypeField } from "@deterministic-code/deterministic-generators-common/parser";
import { createCasing } from "../../../code-generators/src/sql/common/default-casing.ts";
import { buildLiveTables, type SqlTable } from "../../../code-generators/src/sql/common/sql-schema.ts";
import { schemaDiff } from "../../../code-generators/src/sql/common/schema-diff.ts";
import "./register-sql-generate.ts";

const field = (name: string, extra: Partial<TypeField> = {}): TypeField => ({
  name,
  type: extra.type ?? (name === "id" ? "integer" : "string"),
  kind: "primitive",
  base: extra.type ?? (name === "id" ? "integer" : "string"),
  isArray: false,
  isNullable: extra.isNullable ?? false,
  ...extra,
});

const table = (name: string, fields: TypeField[]): SqlTable => ({
  name,
  tags: ["datasource_type"],
  kind: "inherit",
  inherits: "set",
  fields,
  indexes: [],
  uniqueIndexFields: [],
  overlays: [],
});

const snap = (tables: SqlTable[]) => {
  const naming = createCasing({}).naming;
  return { tables: buildLiveTables(tables, naming), naming };
};

describe("schemaDiff postgres", () => {
  it("keeps alter column when a column type changes", () => {
    const before = table("user", [field("id", { isId: true }), field("age", { type: "integer" })]);
    const after = table("user", [field("id", { isId: true }), field("age", { type: "string" })]);
    const statements = schemaDiff("postgres", snap([before]), snap([after]));
    assert.equal(statements.some((item) => item.constructor.name === "AlterTableStatement"), true);
    assert.equal(statements.some((item) => item.constructor.name === "RebuildTableStatement"), false);
  });
});
