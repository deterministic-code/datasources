import { describe, expect, it } from "vitest";
import { DefaultStatementTextRenderer, SelectInsertedIdStatement } from "../index.ts";

describe("SelectInsertedIdStatement", () => {
  it("requires a provider implementation", () => {
    const statement = new SelectInsertedIdStatement("users", "id");
    expect(() => DefaultStatementTextRenderer.instance.render(statement)).toThrow(
      "provider must implement SelectInsertIdStatement",
    );
  });
});
