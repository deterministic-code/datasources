import { describe, expect, it } from "vitest";
import { collectAsync } from "../../collectAsync.ts";
import { ListCRUDRepository } from "../listCRUDRepository.ts";
import { ListReadonlyRepository } from "../listReadonlyRepository.ts";
import { Address } from "./Address.ts";
import { AddressType } from "./AddressType.ts";
import { Contact } from "./Contact.ts";
import { ContactSource } from "./ContactSource.ts";
import { ContactType } from "./ContactType.ts";
import { ContactsService } from "./ContactsService.ts";
import { Phone } from "./Phone.ts";
import { PhoneType } from "./PhoneType.ts";

describe("ContactsService", () => {
  it("fills names and child lists", async () => {
    const addresses = new ListCRUDRepository<Address>([
      Object.assign(new Address(), {
        id: 11,
        contact_id: 9,
        line1: "other",
        city: "x",
      }),
    ]);
    const service = new ContactsService(
      new ListCRUDRepository<Contact>(),
      new ListReadonlyRepository([
        Object.assign(new ContactType(), { id: 2, name: "person" }),
      ]),
      new ListReadonlyRepository([
        Object.assign(new ContactSource(), { id: 3, name: "import" }),
      ]),
      new ListReadonlyRepository([
        Object.assign(new AddressType(), { id: 1, name: "Home" }),
      ]),
      new ListReadonlyRepository([
        Object.assign(new PhoneType(), { id: 2, name: "Work" }),
      ]),
      addresses,
      new ListCRUDRepository<Phone>(),
    );
    await service.create(
      Object.assign(new Contact(), {
        contact_type_id: 2,
        contact_source_id: 3,
        first_name: "Ada",
        addresses: [
          Object.assign(new Address(), {
            line1: "1 Main",
            city: "Paris",
            address_type_id: 1,
          }),
        ],
        phones: [Object.assign(new Phone(), { number: "555", phone_type_id: 2 })],
      }),
    );
    const contact = (await collectAsync(service.read()))[0];
    expect(contact?.contact_type_name).toBe("person");
    expect(contact?.contact_source_name).toBe("import");
    expect(contact?.addresses).toHaveLength(1);
    expect(contact?.addresses[0]?.line1).toBe("1 Main");
    expect(contact?.addresses[0]?.address_type_name).toBe("Home");
    expect(contact?.phones[0]?.number).toBe("555");
    expect(contact?.phones[0]?.phone_type_name).toBe("Work");
  });
});
