import assert from "node:assert/strict";
import { describe, it } from "vitest";
import type { TypeField } from "@deterministic-code/deterministic-generators-common/parser";
import { createCasing } from "../../../code-generators/src/sql/common/default-casing.ts";
import { buildLiveTables, type SqlTable } from "../../../code-generators/src/sql/common/sql-schema.ts";
import { renderSql } from "@deterministic-code/datasources-code-generators/common/sql-dialect";
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

describe("schemaDiff mysql rebuild option", () => {
  it("keeps alter column unless datasource.rebuild_table includes mysql", () => {
    const before = table("user", [field("id", { isId: true }), field("age", { type: "integer" })]);
    const after = table("user", [field("id", { isId: true }), field("age", { type: "string" })]);
    const statements = schemaDiff("mysql", snap([before]), snap([after]));
    assert.equal(statements.some((item) => item.constructor.name === "AlterTableStatement"), true);
    assert.equal(statements.some((item) => item.constructor.name === "RebuildTableStatement"), false);
    const rebuilt = schemaDiff("mysql", snap([before]), snap([after]), {
      "datasource.rebuild_table": "mysql",
    });
    assert.equal(rebuilt.some((item) => item.constructor.name === "RebuildTableStatement"), true);
    assert.equal(rebuilt.some((item) => item.constructor.name === "AlterTableStatement"), false);
    const sql = rebuilt.map((statement) => renderSql("mysql", statement));
    assert.equal(sql.some((item) => item.includes("RENAME TABLE") && item.includes("users__new")), true);
  });
});
