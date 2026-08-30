import { describe, expect, it } from "vitest";
import { Mapper } from "./Mapper.ts";

describe("Mapper", () => {
  it("to and from are the name", () => {
    const mapper = new Mapper("users", "sql", "typescript");
    expect(mapper.name).toBe("users");
    expect(mapper.source).toBe("sql");
    expect(mapper.destination).toBe("typescript");
    expect(mapper.to("user_accounts")).toBe("user_accounts");
    expect(mapper.from("UserAccounts")).toBe("UserAccounts");
  });
});
