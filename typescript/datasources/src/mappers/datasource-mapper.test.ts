import { describe, expect, it } from "vitest";
import { DatasourceMapper } from "./DatasourceMapper.ts";

describe("DatasourceMapper", () => {
  it("maps names the caller supplies and leaves the rest unchanged", () => {
    const mapping = DatasourceMapper.mapping({
      source: "sql",
      destination: "typescript",
      names: { user_accounts: "userAccounts", created_at: "createdAt" },
    });
    expect(mapping.source).toBe("sql");
    expect(mapping.destination).toBe("typescript");
    expect(mapping.names).toEqual({
      user_accounts: "userAccounts",
      created_at: "createdAt",
    });
    expect(mapping.to("user_accounts")).toBe("userAccounts");
    expect(mapping.to("id")).toBe("id");
    expect(mapping.from("createdAt")).toBe("created_at");
    expect(mapping.from("id")).toBe("id");
  });

  it("is identity when no names are given", () => {
    const mapping = DatasourceMapper.mapping({
      source: "sql",
      destination: "go",
    });
    expect(mapping.to("user_name")).toBe("user_name");
    expect(mapping.from("UserName")).toBe("UserName");
  });
});
