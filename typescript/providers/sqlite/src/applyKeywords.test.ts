import assert from "node:assert/strict";
import { describe, it } from "vitest";
import { SqliteDriver } from "./SqliteDriver.ts";

const lower = (word: string): string => word.toLowerCase();

describe("SqliteDriver.applyKeywords", () => {
  it("skips double-quoted identifiers", () => {
    assert.equal(
      SqliteDriver.applyKeywords('CREATE TABLE "CREATE" ("TABLE")', lower),
      'create table "CREATE" ("TABLE")',
    );
    assert.equal(
      SqliteDriver.applyKeywords('SELECT "a""CREATE""b" FROM t', lower),
      'select "a""CREATE""b" from t',
    );
  });

  it("skips strings and comments", () => {
    assert.equal(
      SqliteDriver.applyKeywords("INSERT INTO t VALUES ('CREATE', 'NOT NULL', 'it''s CREATE')", lower),
      "insert into t values ('CREATE', 'NOT NULL', 'it''s CREATE')",
    );
    assert.equal(
      SqliteDriver.applyKeywords("-- CREATE TABLE stays\nDROP TABLE t;", lower),
      "-- CREATE TABLE stays\ndrop table t;",
    );
    assert.equal(
      SqliteDriver.applyKeywords("SELECT /* CREATE TABLE */ 1 FROM t", lower),
      "select /* CREATE TABLE */ 1 from t",
    );
  });

  it("does not case a keyword that is only a prefix of an identifier", () => {
    assert.equal(
      SqliteDriver.applyKeywords("CREATED NOTES ONZONE", lower),
      "CREATED NOTES ONZONE",
    );
  });
});
