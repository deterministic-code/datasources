import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { asContactsView } from "../../../datasources/src/tests/contact/asContactsView.ts";
import { contactsParity } from "../../../datasources/src/tests/contact/contactsParity.ts";
import { expectContactsView } from "../../../datasources/src/tests/contact/expectContactsView.ts";
import { logMapped } from "../../../datasources/src/tests/contact/logMapped.ts";
import { addExtraContact } from "../../../datasources/src/tests/contact/extraContact.ts";
import { contactsServiceOf } from "../../../datasources/src/tests/contact/contactsServiceOf.ts";
import { seedContactCount, seedContactsSql } from "../../../datasources/src/tests/contact/seedContacts.ts";
import { specVerbose } from "../../../sql-spec-runner.ts";
import {
  loadBetterSqlite3Driver,
  SqliteCommand,
  SqliteConnection,
} from "../src/index.ts";

const schemaPath = join(
  dirname(fileURLToPath(import.meta.url)),
  "../../../datasources/src/tests/contact/sqlite.sql",
);

const statementsOf = (sql: string) =>
  sql
    .split(";")
    .map((part) => part.trim())
    .filter(Boolean);

describe("contacts readonly repository", () => {
  it("matches enriched ContactsService to FlatToTypeMapper on the same seed", async () => {
    const connection = new SqliteConnection({
      filename: ":memory:",
      driver: await loadBetterSqlite3Driver(":memory:"),
    });
    await connection.open();
    const command = new SqliteCommand(connection);
    for (const sql of [...statementsOf(readFileSync(schemaPath, "utf8")), ...seedContactsSql()])
      await command.execute(sql);

    const { eager, enriched } = await contactsParity(connection, specVerbose());
    const mapped = eager.find((contact) => contact.id === 10);
    if (specVerbose() && mapped) logMapped(mapped);

    expectContactsView(eager, seedContactCount);
    expectContactsView(enriched, seedContactCount);
    expect(mapped?.contact_source_name).toBeTruthy();
    expect(mapped?.contact_type_name).toBeTruthy();
    expect(new Set(mapped?.addresses.map((address) => address.id)).size).toBe(2);
    expect(asContactsView(enriched)).toEqual(asContactsView(eager));

    const extra = await addExtraContact(contactsServiceOf(connection));
    expect(extra.first_name).toBe("Extra");
    expect(extra.addresses).toHaveLength(1);
    expect(extra.addresses[0]?.line1).toBe("9 Extra");
    expect(extra.addresses[0]?.contact_id).toBe(extra.id);
    expect(extra.phones[0]?.number).toBe("555-extra");
    expect(extra.phones[0]?.contact_id).toBe(extra.id);
  });
});
