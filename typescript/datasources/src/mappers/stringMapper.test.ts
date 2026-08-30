import { describe, expect, it } from "vitest";
import type { IMapper } from "./IMapper.ts";
import type { ISourceDestinationMapper } from "./ISourceDestinationMapper.ts";
import { StringMapper } from "./StringMapper.ts";
import { StringSourceDestinationMapper } from "./StringSourceDestinationMapper.ts";

describe("StringMapper", () => {
  it("to and from are the name", () => {
    const mapping: IMapper<string> = new StringMapper();
    expect(mapping.to("user_accounts")).toBe("user_accounts");
    expect(mapping.from("UserAccounts")).toBe("UserAccounts");
  });
});

describe("StringSourceDestinationMapper", () => {
  it("source and destination", () => {
    const mapper = new StringSourceDestinationMapper("sql", "typescript");
    const labels: ISourceDestinationMapper = mapper;
    const mapping: IMapper<string> = mapper;
    expect(labels.source).toBe("sql");
    expect(labels.destination).toBe("typescript");
    expect(mapping.to("user_accounts")).toBe("user_accounts");
    expect(mapping.from("UserAccounts")).toBe("UserAccounts");
  });
});
