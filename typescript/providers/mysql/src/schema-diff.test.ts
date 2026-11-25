import assert from "node:assert/strict";
import { describe, it } from "vitest";
import type { TypeField } from "@deterministic-code/deterministic-generators-common/parser";
import {
  CreateIndexStatement,
  CreateTriggerStatement,
  ForeignKey,
  PrimaryKey,
  UniqueConstraint,
  type IStatement,
} from "@deterministic-code/datasources";
import { createCasing } from "../../../code-generators/src/sql/common/default-casing.ts";
import { buildLiveTables, type SqlTable } from "../../../code-generators/src/sql/common/sql-schema.ts";
import { renderSql } from "@deterministic-code/datasources-code-generators/common/sql-dialect";
import { schemaDiff } from "../../../code-generators/src/sql/common/schema-diff.ts";
import { fitMysqlIdentifier } from "./MysqlIdentifier.ts";
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

  it("rebuilds a mysql audit table trigger when a foreign key target changes", () => {
    const users = table("user", [field("id", { isId: true })]);
    const accounts = table("account", [field("id", { isId: true })]);
    const audit = [
      field("id", { isId: true }),
      field("created", { type: "datetime" }),
      field("updated", { type: "datetime" }),
    ];
    const before = table("order", [...audit, field("user_id", { type: "integer", references: "user.id" })]);
    const after = table("order", [...audit, field("user_id", { type: "integer", references: "account.id" })]);
    const rebuilt = schemaDiff("mysql", snap([users, before]), snap([accounts, after]), {
      "datasource.rebuild_table": "mysql",
    });
    const sql = rebuilt.map((statement) => renderSql("mysql", statement));
    assert.equal(sql.some((item) => item.includes("RENAME TABLE") && item.includes("orders__new")), true);
    assert.equal(sql.some((item) => item.includes("CREATE TRIGGER")), true);
  });

  it("uses the same short index, foreign key, trigger, and constraint names in down migrations", () => {
    const entity = "game_journey_setting_category_preference_option_value";
    const fkField = `${entity}_id`;
    const naming = createCasing({}).naming;
    const index = CreateIndexStatement.nameOf(entity, naming, `${entity}_id`, "index");
    const trigger = CreateTriggerStatement.nameOf(entity, naming);
    const foreignKey = ForeignKey.nameOf(entity, naming, fkField, "foreign_key");
    const primaryKey = PrimaryKey.nameOf(entity, naming, "primary_key");
    const unique = UniqueConstraint.nameOf(entity, naming, "external_reference_code", "unique_constraint");
    for (const name of [index, trigger, foreignKey, primaryKey, unique]) assert.ok(name.length > 64, name);

    const parent = table("category_type", [field("id", { isId: true })]);
    const fields = [
      field("id", { isId: true }),
      field("created", { type: "datetime" }),
      field("updated", { type: "datetime" }),
      field(fkField, { type: "integer", references: "category_type.id" }),
      field("external_reference_code"),
    ];
    const indexes = [{ name: `${entity}_id`, fields: [fkField], isUnique: false }];
    const plain = table(entity, fields);
    plain.indexes = indexes;
    const withUnique = table(entity, fields);
    withUnique.indexes = indexes;
    withUnique.overlays = [{ name: "external_reference_code", isUnique: true }];
    const withoutFk = table(entity, fields.filter((item) => item.name !== fkField));
    withoutFk.indexes = indexes;

    const text = (statements: IStatement[]) => statements.map((statement) => renderSql("mysql", statement)).join("\n");
    const token = (sql: string, pattern: RegExp): string => {
      const found = pattern.exec(sql);
      assert.ok(found?.[1], sql);
      return found[1].replaceAll("`", "");
    };
    const up = text(schemaDiff("mysql", snap([parent]), snap([parent, plain])));
    const down = text(schemaDiff("mysql", snap([parent, plain]), snap([parent])));
    const ident = /([A-Za-z0-9_`-]+)/;
    assert.equal(token(up, new RegExp(`CREATE INDEX ${ident.source} ON`)), fitMysqlIdentifier(index));
    assert.equal(token(down, new RegExp(`DROP INDEX ${ident.source} ON`)), fitMysqlIdentifier(index));
    assert.equal(token(up, new RegExp(`CREATE TRIGGER ${ident.source}`)), fitMysqlIdentifier(trigger));
    assert.equal(token(down, new RegExp(`DROP TRIGGER IF EXISTS ${ident.source}`)), fitMysqlIdentifier(trigger));
    assert.equal(token(up, new RegExp(`CONSTRAINT ${ident.source} FOREIGN KEY`)), fitMysqlIdentifier(foreignKey));
    assert.equal(token(up, new RegExp(`CONSTRAINT ${ident.source} PRIMARY KEY`)), fitMysqlIdentifier(primaryKey));
    assert.equal(up.includes(index), false);
    assert.equal(down.includes(index), false);

    const addUnique = text(schemaDiff("mysql", snap([parent, plain]), snap([parent, withUnique])));
    const dropUnique = text(schemaDiff("mysql", snap([parent, withUnique]), snap([parent, plain])));
    assert.equal(token(addUnique, new RegExp(`CONSTRAINT ${ident.source} UNIQUE`)), fitMysqlIdentifier(unique));
    assert.equal(token(dropUnique, new RegExp(`DROP CONSTRAINT ${ident.source}`)), fitMysqlIdentifier(unique));

    const dropFk = text(schemaDiff("mysql", snap([parent, plain]), snap([parent, withoutFk])));
    assert.equal(token(dropFk, new RegExp(`DROP FOREIGN KEY ${ident.source}`)), fitMysqlIdentifier(foreignKey));
  });
});
