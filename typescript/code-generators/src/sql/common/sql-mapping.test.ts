import assert from "node:assert/strict";
import { describe, it } from "vitest";
import {
  CreateTableStatement,
  CreateTriggerStatement,
  PrimaryKey,
} from "@deterministic-code/datasources";
import type { TypeField } from "@deterministic-code/deterministic-generators-common/parser";
import { createDatasourceNaming } from "@deterministic-code/generators-common/datasource-naming";
import { createCasing } from "./default-casing.ts";
import {
  fieldStem,
  resolveReference,
  tableStem,
  type SqlTable,
} from "./sql-schema.ts";

const field = (name: string): TypeField => ({
  name,
  type: name === "id" || name === "owner_id" ? "integer" : "string",
  kind: "primitive",
  base: name === "id" || name === "owner_id" ? "integer" : "string",
  isArray: false,
  isNullable: false,
});

const table = (
  name: string,
  opts: {
    mapping?: string;
    fields?: { name: string; mapping?: string }[];
  } = {},
): SqlTable => {
  const fields = opts.fields ?? [{ name: "id" }];
  return {
    name,
    tags: ["datasource_type"],
    kind: "inherit",
    inherits: "set",
    fields: fields.map((f) => field(f.name)),
    indexes: [],
    uniqueIndexFields: [],
    overlays: fields
      .filter((f) => f.mapping !== undefined)
      .map((f) => ({ name: f.name, mapping: f.mapping })),
    ...(opts.mapping !== undefined ? { mapping: opts.mapping } : {}),
  };
};

const usersBase = table("users_base", {
  mapping: "user",
  fields: [{ name: "id" }, { name: "email" }],
});
const child = table("child", {
  fields: [
    { name: "id" },
    { name: "owner_id" },
    { name: "email", mapping: "email_address" },
  ],
});
const person = table("person");
const oldReviews = table("old_reviews", { mapping: "OldRvwsTbl" });
const notification = table("notification");
const tables = [usersBase, child, person, oldReviews, notification];

const namingOf = (settings: Record<string, string> = {}) => createCasing(settings).naming;
const tableName = (t: SqlTable, naming = namingOf()) =>
  CreateTableStatement.tableName(tableStem(t), naming);
const columnName = (t: SqlTable, logical: string, naming = namingOf()) =>
  CreateTableStatement.columnName(fieldStem(t, logical), naming);
const constraintName = (t: SqlTable, ...parts: string[]) =>
  PrimaryKey.nameOf(tableStem(t), namingOf(), ...parts);
const triggerName = (t: SqlTable, naming = namingOf()) =>
  CreateTriggerStatement.nameOf(tableStem(t), naming);

describe("isVerbatim", () => {
  const naming = createDatasourceNaming({});
  it("treats snake stems as cased", () => {
    assert.equal(naming.isVerbatim("user"), false);
    assert.equal(naming.isVerbatim("email_address"), false);
  });

  it("treats mixed-case mappings as verbatim", () => {
    assert.equal(naming.isVerbatim("OldRvwsTbl"), true);
    assert.equal(naming.isVerbatim("UsrProfiles"), true);
  });
});

describe("statement names from stems", () => {
  it("maps users_base / user through pluralize", () => {
    assert.equal(tableStem(usersBase), "user");
    assert.equal(tableName(usersBase), "users");
    assert.equal(constraintName(usersBase, "primary_key"), "users_primary_key");
    assert.equal(triggerName(usersBase), "trg_users_updated_at");
  });

  it("keeps the stem when pluralize is off", () => {
    const naming = namingOf({ "datasource.pluralize_datatable_names": "false" });
    assert.equal(tableName(usersBase, naming), "user");
    assert.equal(PrimaryKey.nameOf(tableStem(usersBase), naming, "primary_key"), "user_primary_key");
  });

  it("pluralizes an unmapped type name", () => {
    assert.equal(tableName(person), "people");
    assert.equal(tableName(notification), "notifications");
  });

  it("uses field overlay mapping for columns", () => {
    assert.equal(fieldStem(child, "email"), "email_address");
    assert.equal(columnName(child, "email"), "email_address");
    assert.equal(columnName(child, "owner_id"), "owner_id");
  });

  it("resolves logical FK references to physical names", () => {
    const naming = namingOf();
    assert.deepEqual(resolveReference(tables, naming, { ...field("owner_id"), references: "users_base.id" }), {
      tableName: "users",
      columnName: "id",
    });
    const off = namingOf({ "datasource.pluralize_datatable_names": "false" });
    assert.deepEqual(resolveReference(tables, off, { ...field("owner_id"), references: "users_base.id" }), {
      tableName: "user",
      columnName: "id",
    });
  });

  it("keeps non-snake mappings verbatim", () => {
    assert.equal(tableName(oldReviews), "OldRvwsTbl");
    assert.equal(constraintName(oldReviews, "primary_key"), "OldRvwsTbl_primary_key");
    assert.equal(triggerName(oldReviews), "trg_OldRvwsTbl_updated_at");
  });

  it("matches CreateTableStatement.tableName when mapping is omitted", () => {
    const naming = namingOf();
    assert.equal(tableName(notification, naming), CreateTableStatement.tableName("notification", naming));
  });

  it("throws when an FK target is missing", () => {
    assert.throws(
      () => resolveReference(tables, namingOf(), { ...field("owner_id"), references: "missing.id" }),
      /unknown table "missing"/,
    );
  });
});
