import { describe, expect, it } from "vitest";
import { MemoryDriver } from "@datasources/memory";
import { collectAsync } from "../../collectAsync.ts";
import { CRUDRepository } from "../../CRUDRepository.ts";
import { ReadonlyRepository } from "../../ReadonlyRepository.ts";
import type { Constructable } from "../../converters/Constructable.ts";
import type { IConnection } from "../../IConnection.ts";
import type { Scalar } from "../../builders/Scalar.ts";
import { Address } from "./Address.ts";
import { AddressType } from "./AddressType.ts";
import { columns, contactEntity, snake } from "./contactColumns.ts";
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
    const typesRepo = createLookupRepo(ContactType, types);
    const source: Record<string, Scalar> = { name: "import" };
    const sources = await setup([source]);
    const sourcesRepo = createLookupRepo(ContactSource, sources);
    const home: Record<string, Scalar> = { name: "Home" };
    const addressTypes = await setup([home]);
    const addressTypesRepo = createLookupRepo(AddressType, addressTypes);
    const work: Record<string, Scalar> = { name: "Work" };
    const phoneTypes = await setup([work]);
    const phoneTypesRepo = createLookupRepo(PhoneType, phoneTypes);
    const connection = await setup();
    const contactsRepo = new CRUDRepository(connection, {
      type: Contact,
      entityMapper: contactEntity,
      columnMappings: columns,
      ids: ["Id"],
    });
    const addressesRepo = new CRUDRepository(connection, {
      type: Address,
      entityMapper: snake,
      columnMappings: columns,
      ids: ["Id"],
    });
    const phonesRepo = new CRUDRepository(connection, {
      type: Phone,
      entityMapper: snake,
      columnMappings: columns,
      ids: ["Id"],
    });
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
      FirstName: "Other",
      ContactType: Object.assign(new ContactType(), { Id: person.id }),
      ContactSource: Object.assign(new ContactSource(), { Id: source.id }),
    }));
    expect(other.Id).toBeGreaterThan(0);
    const stray = await addressesRepo.create(Object.assign(new Address(), {
      Contact: Object.assign(new Contact(), { Id: other.Id }),
      Line1: "other",
      City: "x",
      AddressType: Object.assign(new AddressType(), { Id: home.id }),
    }));
    expect(stray.Id).toBeGreaterThan(0);
    const created = await service.create(Object.assign(new Contact(), {
      ContactType: Object.assign(new ContactType(), { Id: person.id }),
      ContactSource: Object.assign(new ContactSource(), { Id: source.id }),
      FirstName: "Ada",
      Addresses: [Object.assign(new Address(), {
        Line1: "1 Main",
        City: "Paris",
        AddressType: Object.assign(new AddressType(), { Id: home.id }),
      })],
      Phones: [Object.assign(new Phone(), {
        Number: "555",
        PhoneType: Object.assign(new PhoneType(), { Id: work.id }),
      })],
    }));
    expect(created.Id).toBeGreaterThan(0);
    const contact = (await collectAsync(service.read())).find((item) => item.FirstName === "Ada");
    expect(contact?.Id).toBe(created.Id);
    expect(contact?.Id).not.toBe(other.Id);
    expect(contact?.ContactType.Name).toBe("person");
    expect(contact?.ContactSource.Name).toBe("import");
    expect(contact?.Addresses).toHaveLength(1);
    expect(contact?.Addresses[0]?.Id).toBeGreaterThan(0);
    expect(contact?.Addresses[0]?.Line1).toBe("1 Main");
    expect(contact?.Addresses[0]?.AddressType.Name).toBe("Home");
    expect(contact?.Phones[0]?.Id).toBeGreaterThan(0);
    expect(contact?.Phones[0]?.Number).toBe("555");
    expect(contact?.Phones[0]?.PhoneType.Name).toBe("Work");
  });
});

const createLookupRepo = <T extends object>(type: Constructable<T>, connection: IConnection) =>
  new ReadonlyRepository(connection, { type, entityMapper: snake, columnMappings: columns });

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
