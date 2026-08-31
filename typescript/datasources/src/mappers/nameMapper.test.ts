import { describe, expect, it } from "vitest";
import type { IMapper } from "./IMapper.ts";
import type { ISourceDestinationMapper } from "./ISourceDestinationMapper.ts";
import { NameSourceDestinationMapper } from "./NameSourceDestinationMapper.ts";
import { TransformMapper } from "./TransformMapper.ts";

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

type User = { id: number; name: string };
type Contacts = { email: string; phone: string };
type Nested = { user: User; contacts: Contacts };
type Flat = {
  userId: number;
  userName: string;
  contactEmail: string;
  contactPhone: string;
};

describe("TransformMapper", () => {
  it("flattens and unflattens user and contacts", () => {
    const nested: Nested = {
      user: { id: 1, name: "Ada" },
      contacts: { email: "ada@x.dev", phone: "555" },
    };
    const mapper = new TransformMapper<Nested, Flat>(
      (n) => ({
        userId: n.user.id,
        userName: n.user.name,
        contactEmail: n.contacts.email,
        contactPhone: n.contacts.phone,
      }),
      (f) => ({
        user: { id: f.userId, name: f.userName },
        contacts: { email: f.contactEmail, phone: f.contactPhone },
      }),
    );
    const flat = mapper.map(nested);
    expect(flat).toEqual({
      userId: 1,
      userName: "Ada",
      contactEmail: "ada@x.dev",
      contactPhone: "555",
    });
    expect(mapper.reverseMap(flat)).toEqual(nested);
  });
});
