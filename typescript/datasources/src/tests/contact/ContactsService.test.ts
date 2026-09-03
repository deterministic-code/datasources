import { describe, expect, it } from "vitest";
import { collectAsync } from "../../collectAsync.ts";
import { ListReadonlyRepository } from "../listReadonlyRepository.ts";
import { Address } from "./Address.ts";
import { Contact } from "./Contact.ts";
import { ContactSource } from "./ContactSource.ts";
import { ContactType } from "./ContactType.ts";
import { ContactsService } from "./ContactsService.ts";
import { Phone } from "./Phone.ts";

describe("ContactsService", () => {
  it("fills names and child lists", async () => {
    const service = new ContactsService(
      new ListReadonlyRepository([
        Object.assign(new Contact(), {
          id: 1,
          contact_type_id: 2,
          contact_source_id: 3,
          first_name: "Ada",
        }),
      ]),
      new ListReadonlyRepository([
        Object.assign(new ContactType(), { id: 2, name: "person" }),
      ]),
      new ListReadonlyRepository([
        Object.assign(new ContactSource(), { id: 3, name: "import" }),
      ]),
      new ListReadonlyRepository([
        Object.assign(new Address(), {
          id: 10,
          contact_id: 1,
          line1: "1 Main",
          city: "Paris",
        }),
        Object.assign(new Address(), {
          id: 11,
          contact_id: 9,
          line1: "other",
          city: "x",
        }),
      ]),
      new ListReadonlyRepository([
        Object.assign(new Phone(), { id: 20, contact_id: 1, number: "555" }),
      ]),
    );
    const contact = (await collectAsync(service.get()))[0];
    expect(contact?.contact_type_name).toBe("person");
    expect(contact?.contact_source_name).toBe("import");
    expect(contact?.addresses).toHaveLength(1);
    expect(contact?.addresses[0]?.line1).toBe("1 Main");
    expect(contact?.phones[0]?.number).toBe("555");
  });
});
