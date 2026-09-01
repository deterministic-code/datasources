import { describe, expect, it } from "vitest";
import { TypeConverter } from "../converters/TypeConverter.ts";
import { TransformConverterMapper } from "./TransformConverterMapper.ts";

class Wrap extends TypeConverter<string, string> {
  convert = (from: string) => `[${from}]`;
  reverseConvert = (to: string) => to.replace(/^\[|\]$/g, "");
}

describe("TransformConverterMapper", () => {
  it("converts both ways without a filter", () => {
    const mapper = new TransformConverterMapper(new Wrap());
    expect(mapper.map("ab")).toBe("[ab]");
    expect(mapper.reverseMap("[ab]")).toBe("ab");
  });

  it("converts when the filter is true", () => {
    const mapper = new TransformConverterMapper(new Wrap(), (s) => s.includes("ab"));
    expect(mapper.map("ab")).toBe("[ab]");
    expect(mapper.reverseMap("[ab]")).toBe("ab");
  });

  it("is identity when the filter is false", () => {
    const mapper = new TransformConverterMapper(new Wrap(), (s) => s.includes("ab"));
    expect(mapper.map("a")).toBe("a");
    expect(mapper.reverseMap("[a]")).toBe("[a]");
  });

  type Address = { street: string; city: string; country: string };
  type Contact = { email: string; phone: string; address: Address };
  type User = { username: string; displayName: string; contact: Contact };
  type Payload = string[] | User;

  class UserFromCells extends TypeConverter<Payload, Payload> {
    convert = (from: Payload): Payload => {
      if (!Array.isArray(from)) return from;
      const [username, displayName, email, phone, street, city, country] = from;
      if (from.length !== 7) throw new Error("expected 7 cells");
      return {
        username,
        displayName,
        contact: { email, phone, address: { street, city, country } },
      };
    };
    reverseConvert = (to: Payload): Payload =>
      Array.isArray(to)
        ? to
        : [
            to.username,
            to.displayName,
            to.contact.email,
            to.contact.phone,
            to.contact.address.street,
            to.contact.address.city,
            to.contact.address.country,
          ];
  }

  it("expands flat cells into a nested user", () => {
    const cells = [
      "ada",
      "Ada Lovelace",
      "ada@example.com",
      "+44 20 7946 0958",
      "10 Downing St",
      "London",
      "UK",
    ];
    const expected: User = {
      username: "ada",
      displayName: "Ada Lovelace",
      contact: {
        email: "ada@example.com",
        phone: "+44 20 7946 0958",
        address: { street: "10 Downing St", city: "London", country: "UK" },
      },
    };
    const mapper = new TransformConverterMapper(new UserFromCells());
    expect(mapper.map(cells)).toEqual(expected);
    expect(mapper.reverseMap(expected)).toEqual(cells);
  });

  type Account = {
    id: number;
    username: string;
    displayName: string;
    isEnabled: boolean;
    created: Date;
    contact: Contact;
  };
  type AccountPayload = string[] | Account;

  class AccountFromCells extends TypeConverter<AccountPayload, AccountPayload> {
    convert = (from: AccountPayload): AccountPayload => {
      if (!Array.isArray(from)) return from;
      const [id, username, displayName, enabled, created, email, phone, street, city] = from;
      if (from.length !== 10) throw new Error("expected 10 cells");
      return {
        id: Number(id),
        username,
        displayName,
        isEnabled: enabled === "1",
        created: new Date(created),
        contact: { email, phone, address: { street, city, country: "" } },
      };
    };
    reverseConvert = (to: AccountPayload): AccountPayload =>
      Array.isArray(to)
        ? to
        : [
            String(to.id),
            to.username,
            to.displayName,
            to.isEnabled ? "1" : "0",
            to.created.toISOString(),
            to.contact.email,
            to.contact.phone,
            to.contact.address.street,
            to.contact.address.city,
            "",
          ];
  }

  const isEnabled = (payload: AccountPayload) =>
    Array.isArray(payload) ? payload[3] === "1" : payload.isEnabled;

  it("converts types and filters rows", () => {
    const ada = [
      "7",
      "ada",
      "Ada Lovelace",
      "1",
      "2020-01-02T03:04:05.000Z",
      "ada@example.com",
      "+44",
      "10 Downing St",
      "London",
      "secret-note",
    ];
    const guest = [
      "0",
      "guest",
      "Guest",
      "0",
      "2020-01-01T00:00:00.000Z",
      "guest@example.com",
      "",
      "1 Road",
      "Paris",
      "ignore-me",
    ];
    const expected: Account = {
      id: 7,
      username: "ada",
      displayName: "Ada Lovelace",
      isEnabled: true,
      created: new Date("2020-01-02T03:04:05.000Z"),
      contact: {
        email: "ada@example.com",
        phone: "+44",
        address: { street: "10 Downing St", city: "London", country: "" },
      },
    };
    const mapper = new TransformConverterMapper(new AccountFromCells(), isEnabled);
    expect(mapper.map(ada)).toEqual(expected);
    expect(mapper.reverseMap(expected)).toEqual([
      "7",
      "ada",
      "Ada Lovelace",
      "1",
      "2020-01-02T03:04:05.000Z",
      "ada@example.com",
      "+44",
      "10 Downing St",
      "London",
      "",
    ]);
    expect(mapper.map(guest)).toBe(guest);
    expect(mapper.reverseMap(guest)).toBe(guest);
  });
});
