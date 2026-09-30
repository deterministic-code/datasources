import { readFileSync } from "node:fs";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { collectAsync } from "../../../datasources/src/collectAsync.ts";
import { asContactsView } from "../../../datasources/src/tests/contact/asContactsView.ts";
import { contactsJoinQuery } from "../../../datasources/src/tests/contact/contactsJoinQuery.ts";
import { contactsParity } from "../../../datasources/src/tests/contact/contactsParity.ts";
import { contactsView } from "../../../datasources/src/tests/contact/contactsView.ts";
import { contactLookupIds } from "../../../datasources/src/tests/contact/contactLookupIds.ts";
import { contactWith, expectContactsView } from "../../../datasources/src/tests/contact/expectContactsView.ts";
import { logMapped } from "../../../datasources/src/tests/contact/logMapped.ts";
import { addExtraContact, extraContact } from "../../../datasources/src/tests/contact/extraContact.ts";
import { contactsServiceOf } from "../../../datasources/src/tests/contact/contactsServiceOf.ts";
import { Contact } from "../../../datasources/src/tests/contact/Contact.ts";
import { ContactBase } from "../../../datasources/src/tests/contact/ContactBase.ts";
import { ContactSource } from "../../../datasources/src/tests/contact/ContactSource.ts";
import { ContactType } from "../../../datasources/src/tests/contact/ContactType.ts";
import { Address } from "../../../datasources/src/tests/contact/Address.ts";
import { AddressType } from "../../../datasources/src/tests/contact/AddressType.ts";
import { Phone } from "../../../datasources/src/tests/contact/Phone.ts";
import { PhoneType } from "../../../datasources/src/tests/contact/PhoneType.ts";
import { fieldNamesOf } from "../../../datasources/src/createCRUDRepositoryOptions.ts";
import { Operator, Where } from "@datasources/core";
import { seedContactCount, seedContactsSql } from "../../../datasources/src/tests/contact/seedContacts.ts";
import { repoRoot, specVerbose } from "../../../sql-spec-runner.ts";
import { execute, Native } from "@datasources/core";
import { tables } from "../../../schema-spec.ts";
import { FlatToNestedMapper, CRUDRepository } from "../../../datasources/src/index.ts";
import type { Cells } from "../../../datasources/src/converters/Cell.ts";
import { SqliteCommand } from "../src/command.ts";
import { SqliteConnection } from "../src/connection.ts";

const quote = (id: string): string => `"${id.replaceAll('"', '""')}"`;
const seedPath = join(repoRoot(), "specs/contacts/sqlite.sql");

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
    for (const statement of tables("contacts", quote))
      await execute(command, statement);
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
    expect(extra.first_name).toBe("Extra");
    expect(extra.addresses).toHaveLength(1);
    expect(extra.addresses[0]?.line1).toBe("9 Extra");
    expect(extra.addresses[0]?.contact_id).toBe(extra.id);
    expect(extra.phones[0]?.number).toBe("555-extra");
    expect(extra.phones[0]?.contact_id).toBe(extra.id);
    expect(extra.addresses[0]?.address_type_name).toBe("Home");
    expect(extra.phones[0]?.phone_type_name).toBe("Mobile");
  });

  it("maps join rows to nested contacts", async () => {
    const connection = SqliteConnection.from({ DATASOURCE_URL: "sqlite:///:memory:" });
    await connection.open();
    const command = new SqliteCommand(connection);
    for (const statement of tables("contacts", quote))
      await execute(command, statement);
    for (const sql of [...statementsOf(readFileSync(seedPath, "utf8")), ...seedContactsSql()])
      await command.executeNative(new Native(sql));

    const contacts = new FlatToNestedMapper(contactsView).map(
      await collectAsync(
        new CRUDRepository<Cells>(connection, { entity: "contacts_base" }).read(contactsJoinQuery()),
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
    for (const statement of tables("contacts", quote))
      await execute(command, statement);
    for (const sql of statementsOf(readFileSync(seedPath, "utf8")))
      await command.executeNative(new Native(sql));

    const contactsRepo = new CRUDRepository(connection, {
      type: Contact,
      entity: "contacts_base",
      readProperties: fieldNamesOf(ContactBase),
    });
    const typesRepo = new CRUDRepository(connection, { type: ContactType, entity: "contact_type" });
    const sourcesRepo = new CRUDRepository(connection, { type: ContactSource, entity: "contact_source" });
    const addressTypesRepo = new CRUDRepository(connection, { type: AddressType, entity: "address_type" });
    const phoneTypesRepo = new CRUDRepository(connection, { type: PhoneType, entity: "phone_type" });
    const addressesRepo = new CRUDRepository(connection, {
      type: Address,
      entity: "address",
      readProperties: [
        "id", "uuid", "created", "updated", "version", "contact_id", "address_type_id",
        "line1", "line2", "city", "region", "postal_code", "country",
      ],
    });
    const phonesRepo = new CRUDRepository(connection, {
      type: Phone,
      entity: "phone",
      readProperties: [
        "id", "uuid", "created", "updated", "version", "contact_id", "phone_type_id", "number",
      ],
    });
    addressesRepo.addEnriched(
      { into: "address_type_name", from: "address_type_id", by: "id", select: "name" },
      addressTypesRepo,
    );
    phonesRepo.addEnriched(
      { into: "phone_type_name", from: "phone_type_id", by: "id", select: "name" },
      phoneTypesRepo,
    );
    contactsRepo.addEnriched(
      { into: "contact_type_name", from: "contact_type_id", by: "id", select: "name" },
      typesRepo,
    );
    contactsRepo.addEnriched(
      { into: "contact_source_name", from: "contact_source_id", by: "id", select: "name" },
      sourcesRepo,
    );
    contactsRepo.addEnriched({ into: "addresses", from: "id", by: "contact_id", many: true }, addressesRepo);
    contactsRepo.addEnriched({ into: "phones", from: "id", by: "contact_id", many: true }, phonesRepo);

    const created = await contactsRepo.create(extraContact(await contactLookupIds(connection)));
    const loaded = await contactsRepo.readSingle(new Where(new Operator.Equals("id", created.id)));
    expect(created.id).toBeGreaterThan(0);
    expect(loaded?.first_name).toBe("Extra");
    expect(loaded?.contact_type_name).toBe("Person");
    expect(loaded?.contact_source_name).toBe("Web");
    expect(loaded?.addresses).toHaveLength(1);
    expect(loaded?.addresses[0]?.line1).toBe("9 Extra");
    expect(loaded?.addresses[0]?.address_type_name).toBe("Home");
    expect(loaded?.addresses[0]?.contact_id).toBe(loaded?.id);
    expect(loaded?.phones).toHaveLength(1);
    expect(loaded?.phones[0]?.number).toBe("555-extra");
    expect(loaded?.phones[0]?.phone_type_name).toBe("Mobile");
    expect(loaded?.phones[0]?.contact_id).toBe(loaded?.id);
  });
});
