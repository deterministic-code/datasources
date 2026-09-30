import { describe, expect, it } from "vitest";
import { TransformMapper } from "./TransformMapper.ts";

type User = { id: number; name: string };
type Contacts = { email: string; phone: string };
type Nested = { user: User; contacts: Contacts };
type Flat = {
  userId: number;
  userName: string;
  contactEmail: string;
  contactPhone: string;
};
type Contact = { name: string; email: string };
type ContactRow = { contactName: string; contactEmail: string };

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

  it("prefixes a contact field", () => {
    const mapper = new TransformMapper<string, string>(
      (name) => `contact_${name}`,
      (name) => name.slice("contact_".length),
    );
    const mapped = mapper.map("email");
    const reversed = mapper.reverseMap(mapped);
    expect(mapped).toBe("contact_email");
    expect(reversed).toBe("email");
  });

  it("maps a contact name and email", () => {
    const contact: Contact = { name: "Ada", email: "ada@x.dev" };
    const mapper = new TransformMapper<Contact, ContactRow>(
      (item) => ({ contactName: item.name, contactEmail: item.email }),
      (row) => ({ name: row.contactName, email: row.contactEmail }),
    );
    const mapped = mapper.map(contact);
    const reversed = mapper.reverseMap(mapped);
    expect(mapped).toEqual({ contactName: "Ada", contactEmail: "ada@x.dev" });
    expect(reversed).toEqual(contact);
  });
});
