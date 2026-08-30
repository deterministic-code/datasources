import { describe, expect, it } from "vitest";
import type { IMapper } from "./IMapper.ts";
import type { ISourceDestinationMapper } from "./ISourceDestinationMapper.ts";
import { NameSourceDestinationMapper } from "./NameSourceDestinationMapper.ts";

describe("NameMapper", () => {
  it("map and reverseMap are the name", () => {
    const mapping: IMapper<string, string> = new NameSourceDestinationMapper(
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
    const labels: ISourceDestinationMapper = mapper;
    const mapping: IMapper<string, string> = mapper;
    expect(labels.source).toBe("sql");
    expect(labels.destination).toBe("typescript");
    expect(mapping.map("user_accounts")).toBe("user_accounts");
    expect(mapping.reverseMap("UserAccounts")).toBe("UserAccounts");
  });
});
