import { describe, expect, it } from "vitest";
import { ChangeCasingMapper } from "./ChangeCasingMapper.ts";
import { PluralizerMapper } from "./PluralizerMapper.ts";

describe("PluralizerMapper", () => {
  it("pluralizes and lowercases the table name", () => {
    const mapper = new PluralizerMapper();
    expect(mapper.map("User")).toBe("Users");
    expect(mapper.reverseMap("Users")).toBe("User");
  });

  it("maps contacts", () => {
    const mapper = new PluralizerMapper();
    const contact = mapper.map("Contact");
    const address = mapper.map("Address");
    expect(contact).toBe("Contacts");
    expect(address).toBe("Addresses");
  });

  it("reverse maps contacts", () => {
    const mapper = new PluralizerMapper();
    const contact = mapper.reverseMap("Contacts");
    const address = mapper.reverseMap("Addresses");
    expect(contact).toBe("Contact");
    expect(address).toBe("Address");
  });

  it("lowercases then pluralizes", () => {
    const casing = new ChangeCasingMapper("pascalCase", "snakeCase");
    const pluralizer = new PluralizerMapper();
    const contact = pluralizer.map(casing.map("Contact"));
    const address = pluralizer.map(casing.map("Address"));
    const contactName = casing.reverseMap(pluralizer.reverseMap(contact));
    const addressName = casing.reverseMap(pluralizer.reverseMap(address));
    expect(contact).toBe("contacts");
    expect(address).toBe("addresses");
    expect(contactName).toBe("Contact");
    expect(addressName).toBe("Address");
  });
});
