import { describe, expect, it } from "vitest";
import type { INameMapper } from "./INameMapper.ts";
import { NameSourceDestinationMapper } from "./NameSourceDestinationMapper.ts";

describe("INameMapper", () => {
  it("map and reverseMap are the name", () => {
    const mapping: INameMapper = new NameSourceDestinationMapper(
      "sql",
      "typescript",
    );
    expect(mapping.map("user_accounts")).toBe("user_accounts");
    expect(mapping.reverseMap("UserAccounts")).toBe("UserAccounts");
  });
});

describe("NameSourceDestinationMapper", () => {
  it("source and destination", () => {
    const mapper = new NameSourceDestinationMapper("sql", "typescript");
    const mapping: INameMapper = mapper;
    expect(mapper.source).toBe("sql");
    expect(mapper.destination).toBe("typescript");
    expect(mapping.map("user_accounts")).toBe("user_accounts");
    expect(mapping.reverseMap("UserAccounts")).toBe("UserAccounts");
  });
});
