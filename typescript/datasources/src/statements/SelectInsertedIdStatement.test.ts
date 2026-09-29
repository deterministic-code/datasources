import { describe, expect, it } from "vitest";
import { SelectInsertedIdStatement } from "./SelectInsertedIdStatement.ts";

describe("SelectInsertedIdStatement", () => {
  it("throws until a provider formats the select", () => {
    const statement = new SelectInsertedIdStatement("users", "id");
    expect(statement.table).toBe("users");
    expect(statement.column).toBe("id");
    expect(() => statement.getText()).toThrow("provider must implement SelectInsertIdStatement");
  });
});
