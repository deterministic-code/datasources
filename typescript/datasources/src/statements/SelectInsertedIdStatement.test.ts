import { describe, expect, it } from "vitest";
import {
  DefaultStatementTextRenderer,
  SelectInsertedIdStatement,
  SqlSeedStatement,
} from "../index.ts";

describe("SelectInsertedIdStatement", () => {
  it("requires a provider implementation", () => {
    const statement = new SelectInsertedIdStatement("users", "id");
    expect(() => DefaultStatementTextRenderer.instance.render(statement)).toThrow(
      "provider must implement SelectInsertIdStatement",
    );
  });
});

describe("SqlSeedStatement", () => {
  it("is rendered by the SQL driver", () => {
    const statement = new SqlSeedStatement("users", [
      { column: "id", field: { type: "integer" }, value: 1 },
    ]);
    expect(statement.table).toBe("users");
    expect(statement.kind).toBe("insert");
    expect(statement.identity).toBe(false);
    expect(() => DefaultStatementTextRenderer.instance.render(statement)).toThrow(
      "SqlSeedStatement is rendered by the SQL driver.",
    );
  });
});
