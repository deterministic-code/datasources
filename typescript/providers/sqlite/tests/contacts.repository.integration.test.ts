import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import {
  collectAsync,
  ConsoleCommandMiddleware,
  FlatToTypeMapper,
  Repository,
  type Cells,
} from "@datasources/core";
import { contactsJoinQuery } from "../../../datasources/src/tests/contact/contactsJoinQuery.ts";
import { contactsView } from "../../../datasources/src/tests/contact/contactsView.ts";
import { logMapped } from "../../../datasources/src/tests/contact/logMapped.ts";
import { expectContactsView } from "../../../datasources/src/tests/contact/expectContactsView.ts";
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
  it("joins contacts and collapses grouped addresses and phones", async () => {
    const connection = new SqliteConnection({
      filename: ":memory:",
      driver: await loadBetterSqlite3Driver(":memory:"),
    });
    await connection.open();
    const command = new SqliteCommand(connection);
    for (const sql of [...statementsOf(readFileSync(schemaPath, "utf8")), ...seedContactsSql()])
      await command.execute(sql);

    const repo = new Repository<Cells>({ connection, entity: "contacts_base" });
    repo.use(
      new ConsoleCommandMiddleware(undefined, undefined, {
        isVerbose: specVerbose() ? "true" : "false",
      }),
    );
    const mapper = new FlatToTypeMapper(contactsView);
    const contacts = mapper.collapse(await collectAsync(repo.get(contactsJoinQuery())));
    const mapped = contacts.find((contact) => contact.id === 10);
    if (specVerbose() && mapped) logMapped(mapped);

    expectContactsView(contacts, seedContactCount);
    expect(mapped?.contact_source_name).toBeTruthy();
    expect(mapped?.contact_type_name).toBeTruthy();
    expect(new Set(mapped?.addresses.map((address) => address.id)).size).toBe(2);
  });
});
