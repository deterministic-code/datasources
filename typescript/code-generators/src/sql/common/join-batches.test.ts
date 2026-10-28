import assert from "node:assert/strict";
import { describe, it } from "vitest";
import { batchSeparatorOf, joinBatches } from "./join-batches.ts";

describe("batchSeparatorOf", () => {
  it("defaults to GO", () => {
    assert.equal(batchSeparatorOf({}), "GO");
  });

  it("reads datasource.batch_separator", () => {
    assert.equal(batchSeparatorOf({ "datasource.batch_separator": "GO" }), "GO");
    assert.equal(batchSeparatorOf({ "datasource.batch_separator": "" }), "");
  });
});

describe("joinBatches", () => {
  it("joins with a whole-line separator", () => {
    assert.equal(joinBatches(["CREATE TABLE t (id INT);", "CREATE TRIGGER tr"], "GO"), [
      "CREATE TABLE t (id INT);",
      "GO",
      "CREATE TRIGGER tr",
    ].join("\n"));
  });

  it("drops empty chunks", () => {
    assert.equal(joinBatches(["a", "", "  ", "b"], "GO"), "a\nGO\nb");
  });

  it("uses a newline when the separator is blank", () => {
    assert.equal(joinBatches(["a", "b"], ""), "a\nb");
    assert.equal(joinBatches(["a", "b"], "   "), "a\nb");
  });
});
