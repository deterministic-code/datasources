import { describe, expect, it } from "vitest";
import { MemoryDriver } from "@datasources/memory";
import { collectAsync } from "../../collectAsync.ts";
import { CRUDRepository } from "../../CRUDRepository.ts";
import { ReadonlyRepository } from "../../ReadonlyRepository.ts";
import type { IConnection } from "../../IConnection.ts";
import type { Scalar } from "../../builders/Scalar.ts";
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
    const person: Record<string, Scalar> = { name: "person" };
    const types = await setup([person]);
    const typesRepo = new ReadonlyRepository(ContactType, types);
    const source: Record<string, Scalar> = { name: "import" };
    const sources = await setup([source]);
    const sourcesRepo = new ReadonlyRepository(ContactSource, sources);
    const home: Record<string, Scalar> = { name: "Home" };
    const addressTypes = await setup([home]);
    const addressTypesRepo = new ReadonlyRepository(AddressType, addressTypes);
    const work: Record<string, Scalar> = { name: "Work" };
    const phoneTypes = await setup([work]);
    const phoneTypesRepo = new ReadonlyRepository(PhoneType, phoneTypes);
    const connection = await setup();
    const contactsRepo = new CRUDRepository(Contact, connection);
    const addressesRepo = new CRUDRepository(Address, connection);
    const phonesRepo = new CRUDRepository(Phone, connection);
    const service = ContactsService.open(
      contactsRepo,
      typesRepo,
      sourcesRepo,
      addressTypesRepo,
      phoneTypesRepo,
      addressesRepo,
      phonesRepo,
    );
    const other = await service.create(Object.assign(new Contact(), {
      first_name: "Other",
      contact_type_id: person.id,
      contact_source_id: source.id,
    }));
    expect(other.id).toBeGreaterThan(0);
    const stray = await addressesRepo.create(Object.assign(new Address(), {
      contact_id: other.id,
      line1: "other",
      city: "x",
      address_type_id: home.id,
    }));
    expect(stray.id).toBeGreaterThan(0);
    const created = await service.create(Object.assign(new Contact(), {
      contact_type_id: person.id,
      contact_source_id: source.id,
      first_name: "Ada",
      addresses: [Object.assign(new Address(), {
        line1: "1 Main",
        city: "Paris",
        address_type_id: home.id,
      })],
      phones: [Object.assign(new Phone(), { number: "555", phone_type_id: work.id })],
    }));
    expect(created.id).toBeGreaterThan(0);
    const contact = (await collectAsync(service.read())).find((item) => item.first_name === "Ada");
    expect(contact?.id).toBe(created.id);
    expect(contact?.id).not.toBe(other.id);
    expect(contact?.contact_type_name).toBe("person");
    expect(contact?.contact_source_name).toBe("import");
    expect(contact?.addresses).toHaveLength(1);
    expect(contact?.addresses[0]?.id).toBeGreaterThan(0);
    expect(contact?.addresses[0]?.line1).toBe("1 Main");
    expect(contact?.addresses[0]?.address_type_name).toBe("Home");
    expect(contact?.phones[0]?.id).toBeGreaterThan(0);
    expect(contact?.phones[0]?.number).toBe("555");
    expect(contact?.phones[0]?.phone_type_name).toBe("Work");
  });
});

const setup = async (rows: Record<string, Scalar>[] = []): Promise<IConnection> => {
  const connection = new MemoryDriver().connection();
  await connection.open();
  for (const row of rows) {
    const response = await connection.createCommand().execute(
      "insert",
      Object.entries(row).map(([name, value]) => ({ name, value })),
    );
    row.id = response.result as number;
  }
  return connection;
};
