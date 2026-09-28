import { describe, expect, it } from "vitest";
import { InsertedIdStatement } from "./InsertedIdStatement.ts";

describe("InsertedIdStatement", () => {
  it("selects the dialect inserted-id query", () => {
    expect(new InsertedIdStatement("users", "id").getText()).toBe("users.id");
    expect(InsertedIdStatement.select("sqlite", "id")).toBe("SELECT last_insert_rowid() AS id");
    expect(InsertedIdStatement.select("mysql", "id")).toBe("SELECT LAST_INSERT_ID() AS id");
    expect(InsertedIdStatement.select("postgres", "id")).toBe("SELECT lastval() AS id");
    expect(InsertedIdStatement.select("memory", "id")).toBe("");
  });
});
