import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import {
  collectAsync,
  Join,
  Repository,
  StatementBuilder,
  FlatToTypeMapper,
  type Cells,
} from "@datasources/core";
import { contactsView } from "../../../datasources/src/tests/contact/contactsView.ts";
import { seedContactCount, seedContactsSql } from "../../../datasources/src/tests/contact/seedContacts.ts";
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

const contactColumns = [
  "contacts_base.id AS id",
  "contacts_base.first_name AS first_name",
  "contacts_base.last_name AS last_name",
  "contacts_base.email AS email",
  "contact_source.name AS contact_source_name",
  "contact_type.name AS contact_type_name",
  "address.id AS address_id",
  "address.line1 AS address_line1",
  "address.city AS address_city",
  "phone.id AS phone_id",
  "phone.number AS phone_number",
  "phone.label AS phone_label",
];

describe("contacts readonly repository", () => {
  it("joins contacts and collapses grouped addresses and phones", async () => {
    const connection = new SqliteConnection({
      filename: ":memory:",
      driver: await loadBetterSqlite3Driver(":memory:"),
    });
    await connection.open();
    const command = new SqliteCommand(connection);
    for (const sql of [...statementsOf(readFileSync(schemaPath, "utf8")), ...seedContactsSql()])
      await command.execute(sql);

    const query = StatementBuilder.createQuery("contacts_base", contactColumns, undefined, [
      Join.inner("contact_source", "contacts_base.contact_source_id", "contact_source.id"),
      Join.inner("contact_type", "contacts_base.contact_type_id", "contact_type.id"),
      Join.left("address", "contacts_base.id", "address.contact_id"),
      Join.left("phone", "contacts_base.id", "phone.contact_id"),
    ]);
    const repo = new Repository<Cells>({ connection, entity: "contacts_base" });
    const mapper = new FlatToTypeMapper(contactsView);
    const contacts = mapper.collapse(await collectAsync(repo.get(query)));

    expect(contacts).toHaveLength(seedContactCount);
    const ada = contacts.find((contact) => contact.id === 10);
    expect(ada?.addresses).toHaveLength(2);
    expect(ada?.phones).toHaveLength(1);
    expect(ada?.contact_source_name).toBeTruthy();
    expect(ada?.contact_type_name).toBeTruthy();
    expect(new Set(ada?.addresses.map((address) => address.id)).size).toBe(2);
  });
});
