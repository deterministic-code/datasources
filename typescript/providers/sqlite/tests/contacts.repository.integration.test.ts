import { readFileSync } from "node:fs";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { collectAsync } from "../../../datasources/src/collectAsync.ts";
import { asContactsView } from "../../../datasources/src/tests/contact/asContactsView.ts";
import { contactsJoinSelect } from "../../../datasources/src/tests/contact/contactsParity.ts";
import { contactsParity } from "../../../datasources/src/tests/contact/contactsParity.ts";
import { contactsView } from "../../../datasources/src/tests/contact/contactsView.ts";
import { contactLookupIds } from "../../../datasources/src/tests/contact/contactLookupIds.ts";
import { contactWith, expectContactsView } from "../../../datasources/src/tests/contact/expectContactsView.ts";
import { logMapped } from "../../../datasources/src/tests/contact/logMapped.ts";
import { addExtraContact, extraContact } from "../../../datasources/src/tests/contact/extraContact.ts";
import { contactsServiceOf } from "../../../datasources/src/tests/contact/contactsServiceOf.ts";
import { Contact } from "../../../datasources/src/tests/contact/Contact.ts";
import { ContactSource } from "../../../datasources/src/tests/contact/ContactSource.ts";
import { ContactType } from "../../../datasources/src/tests/contact/ContactType.ts";
import { Address } from "../../../datasources/src/tests/contact/Address.ts";
import { AddressType } from "../../../datasources/src/tests/contact/AddressType.ts";
import { Phone } from "../../../datasources/src/tests/contact/Phone.ts";
import { PhoneType } from "../../../datasources/src/tests/contact/PhoneType.ts";
import { columns, contactEntity, joinEntity, snake } from "../../../datasources/src/tests/contact/contactColumns.ts";
import { Operator, Where } from "@deterministic-code/datasources";
import { seedContactCount, seedContactsSql } from "../../../datasources/src/tests/contact/seedContacts.ts";
import { specsDir, specVerbose } from "../../../sql-spec-runner.ts";
import { Native } from "@deterministic-code/datasources";
import { applyContacts } from "../../../contacts-spec.ts";
import { FlatToNestedMapper, CRUDRepository } from "../../../datasources/src/index.ts";
import type { Cells } from "../../../datasources/src/converters/Cell.ts";
import { SqliteCommand } from "../src/command.ts";
import { SqliteConnection } from "../src/connection.ts";
import { SqliteDriver } from "../src/SqliteDriver.ts";

const seedPath = join(specsDir(), "contacts/sqlite.sql");

const statementsOf = (sql: string) =>
  sql
    .split(";")
    .map((part) => part.trim())
    .filter(Boolean);

describe("contacts readonly repository", () => {
  it("matches enriched ContactsService to FlatToNestedMapper on the same seed", async () => {
    const connection = SqliteConnection.from({ DATASOURCE_URL: "sqlite:///:memory:" });
    await connection.open();
    const command = new SqliteCommand(connection);
    await applyContacts(connection, "sqlite", new SqliteDriver());
    for (const sql of [...statementsOf(readFileSync(seedPath, "utf8")), ...seedContactsSql()])
      await command.executeNative(new Native(sql));

    const { eager, enriched } = await contactsParity(connection, specVerbose());
    const mapped = contactWith(eager, 2, 1);
    if (specVerbose()) logMapped(mapped);

    expectContactsView(eager, seedContactCount);
    expectContactsView(enriched, seedContactCount);
    expect(mapped.contact_source_name).toBeTruthy();
    expect(mapped.contact_type_name).toBeTruthy();
    expect(new Set(mapped.addresses.map((address) => address.id)).size).toBe(2);
    expect(asContactsView(enriched)).toEqual(asContactsView(eager));

    const extra = await addExtraContact(
      contactsServiceOf(connection),
      await contactLookupIds(connection),
    );
    expect(extra.FirstName).toBe("Extra");
    expect(extra.Addresses).toHaveLength(1);
    expect(extra.Addresses[0]?.Line1).toBe("9 Extra");
    expect(extra.Addresses[0]?.Contact.Id).toBe(extra.Id);
    expect(extra.Phones[0]?.Number).toBe("555-extra");
    expect(extra.Phones[0]?.Contact.Id).toBe(extra.Id);
    expect(extra.Addresses[0]?.AddressType.Name).toBe("Home");
    expect(extra.Phones[0]?.PhoneType.Name).toBe("Mobile");
  });

  it("maps join rows to nested contacts", async () => {
    const connection = SqliteConnection.from({ DATASOURCE_URL: "sqlite:///:memory:" });
    await connection.open();
    const command = new SqliteCommand(connection);
    await applyContacts(connection, "sqlite", new SqliteDriver());
    for (const sql of [...statementsOf(readFileSync(seedPath, "utf8")), ...seedContactsSql()])
      await command.executeNative(new Native(sql));

    const contacts = new FlatToNestedMapper(contactsView).map(
      await collectAsync(
        new CRUDRepository<Cells>(connection, { entityMapper: joinEntity, propertyMappings: columns }).read(
          contactsJoinSelect(),
        ),
      ),
    );
    const mapped = contactWith(contacts, 2, 1);
    expect(mapped.addresses).toHaveLength(2);
    expect(mapped.contact_source_name).toBeTruthy();
  });

  it("writes enriched contact", async () => {
    const connection = SqliteConnection.from({ DATASOURCE_URL: "sqlite:///:memory:" });
    await connection.open();
    const command = new SqliteCommand(connection);
    await applyContacts(connection, "sqlite", new SqliteDriver());
    for (const sql of statementsOf(readFileSync(seedPath, "utf8")))
      await command.executeNative(new Native(sql));

    const contactsRepo = new CRUDRepository(connection, {
      type: Contact,
      entityMapper: contactEntity,
      propertyMappings: columns,
      ids: ["Id"],
    });
    const typesRepo = new CRUDRepository(connection, { type: ContactType, entityMapper: snake, propertyMappings: columns, ids: ["Id"] });
    const sourcesRepo = new CRUDRepository(connection, { type: ContactSource, entityMapper: snake, propertyMappings: columns, ids: ["Id"] });
    const addressTypesRepo = new CRUDRepository(connection, { type: AddressType, entityMapper: snake, propertyMappings: columns, ids: ["Id"] });
    const phoneTypesRepo = new CRUDRepository(connection, { type: PhoneType, entityMapper: snake, propertyMappings: columns, ids: ["Id"] });
    const addressesRepo = new CRUDRepository(connection, { type: Address, entityMapper: snake, propertyMappings: columns, ids: ["Id"] });
    const phonesRepo = new CRUDRepository(connection, { type: Phone, entityMapper: snake, propertyMappings: columns, ids: ["Id"] });
    addressesRepo.addEnriched({ destinationProperty: "AddressType", sourceProperty: "AddressType", relatedKey: "Id" }, addressTypesRepo);
    phonesRepo.addEnriched({ destinationProperty: "PhoneType", sourceProperty: "PhoneType", relatedKey: "Id" }, phoneTypesRepo);
    contactsRepo.addEnriched({ destinationProperty: "ContactType", sourceProperty: "ContactType", relatedKey: "Id" }, typesRepo);
    contactsRepo.addEnriched({ destinationProperty: "ContactSource", sourceProperty: "ContactSource", relatedKey: "Id" }, sourcesRepo);
    contactsRepo.addEnriched({ destinationProperty: "Addresses", sourceProperty: "Id", relatedKey: "Contact", type: "list" }, addressesRepo);
    contactsRepo.addEnriched({ destinationProperty: "Phones", sourceProperty: "Id", relatedKey: "Contact", type: "list" }, phonesRepo);

    const created = await contactsRepo.create(extraContact(await contactLookupIds(connection)));
    const loaded = await contactsRepo.readSingle(new Where(new Operator.Equals("Id", created.Id)));
    expect(created.Id).toBeGreaterThan(0);
    expect(loaded?.FirstName).toBe("Extra");
    expect(loaded?.ContactType.Name).toBe("Person");
    expect(loaded?.ContactSource.Name).toBe("Web");
    expect(loaded?.Addresses).toHaveLength(1);
    expect(loaded?.Addresses[0]?.Line1).toBe("9 Extra");
    expect(loaded?.Addresses[0]?.AddressType.Name).toBe("Home");
    expect(loaded?.Addresses[0]?.Contact.Id).toBe(loaded?.Id);
    expect(loaded?.Phones).toHaveLength(1);
    expect(loaded?.Phones[0]?.Number).toBe("555-extra");
    expect(loaded?.Phones[0]?.PhoneType.Name).toBe("Mobile");
    expect(loaded?.Phones[0]?.Contact.Id).toBe(loaded?.Id);
  });
});
