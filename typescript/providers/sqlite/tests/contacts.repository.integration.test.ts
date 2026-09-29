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
import { addExtraContact } from "../../../datasources/src/tests/contact/extraContact.ts";
import { contactsServiceOf } from "../../../datasources/src/tests/contact/contactsServiceOf.ts";
import { seedContactCount, seedContactsSql } from "../../../datasources/src/tests/contact/seedContacts.ts";
import { repoRoot, specVerbose } from "../../../sql-spec-runner.ts";
import { execute } from "@datasources/core";
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
      await command.execute(sql);

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
      await command.execute(sql);

    const contacts = new FlatToNestedMapper(contactsView).map(
      await collectAsync(
        new CRUDRepository<Cells>(connection, { entity: "contacts_base" }).read(contactsJoinQuery()),
      ),
    );
    const mapped = contactWith(contacts, 2, 1);
    expect(mapped.addresses).toHaveLength(2);
    expect(mapped.contact_source_name).toBeTruthy();
  });
});
