import { describe, expect, it } from "vitest";
import { PluralizerMapper } from "./PluralizerMapper.ts";

describe("PluralizerMapper", () => {
  it("pluralizes and lowercases the table name", () => {
    const mapper = new PluralizerMapper();
    expect(mapper.map("User")).toBe("users");
    expect(mapper.reverseMap("users")).toBe("user");
  });

  it("maps contacts", () => {
    const mapper = new PluralizerMapper();
    const contact = mapper.map("Contact");
    const address = mapper.map("Address");
    expect(contact).toBe("contacts");
    expect(address).toBe("addresses");
  });

  it("reverse maps contacts", () => {
    const mapper = new PluralizerMapper();
    const contact = mapper.reverseMap("contacts");
    const address = mapper.reverseMap("addresses");
    expect(contact).toBe("contact");
    expect(address).toBe("address");
  });
});
