import { readFileSync } from "node:fs";
import { join } from "node:path";
import { beforeAll, describe, expect, it } from "vitest";
import { asContactsView } from "../../../datasources/src/tests/contact/asContactsView.ts";
import { contactsParity } from "../../../datasources/src/tests/contact/contactsParity.ts";
import { contactLookupIds } from "../../../datasources/src/tests/contact/contactLookupIds.ts";
import { contactWith, expectContactsView } from "../../../datasources/src/tests/contact/expectContactsView.ts";
import { logMapped } from "../../../datasources/src/tests/contact/logMapped.ts";
import { addExtraContact } from "../../../datasources/src/tests/contact/extraContact.ts";
import { contactsServiceOf } from "../../../datasources/src/tests/contact/contactsServiceOf.ts";
import { seedContactCount, seedContactsSql } from "../../../datasources/src/tests/contact/seedContacts.ts";
import { ensureCompose } from "../../../ensure-compose.ts";
import { repoRoot, specVerbose } from "../../../sql-spec-runner.ts";
import { execute, Native } from "@datasources/core";
import { tables } from "../../../schema-spec.ts";
import { PostgresCommand } from "../src/command.ts";
import { PostgresConnection } from "../src/connection.ts";
import type { PgModule } from "../src/i-postgres-driver.ts";

const url = (process.env.POSTGRES_URL ??=
  "postgres://ds:ds@127.0.0.1:5432/datasources");

const quote = (id: string): string => `"${id.replaceAll('"', '""')}"`;
const seedPath = join(repoRoot(), "specs/contacts/postgres.sql");

const statementsOf = (sql: string) =>
  sql
    .split(";")
    .map((part) => part.trim())
    .filter(Boolean);

beforeAll(() => ensureCompose("postgres"), 120_000);

const isolate = async (base: string) => {
  const pg = (await import("pg")) as PgModule;
  const Client = pg.default?.Client ?? pg.Client;
  const schema = `ds_${crypto.randomUUID().replaceAll("-", "").slice(0, 16)}`;
  const admin = new Client({ connectionString: base });
  await admin.connect();
  await admin.query(`CREATE SCHEMA ${schema}`);
  await admin.end();
  return {
    schema,
    cleanup: async () => {
      const drop = new Client({ connectionString: base });
      await drop.connect();
      await drop.query(`DROP SCHEMA ${schema} CASCADE`);
      await drop.end();
    },
  };
};

describe("contacts readonly repository", () => {
  it("matches enriched ContactsService to FlatToNestedMapper on the same seed", async () => {
    const isolated = await isolate(url);
    try {
      const connection = PostgresConnection.from({ DATASOURCE_URL: `${url}${url.includes("?") ? "&" : "?"}schema=${isolated.schema}` });
      await connection.open();
      const command = new PostgresCommand(connection);
      for (const statement of tables("contacts", quote))
        await execute(command, statement);
      for (const sql of [...statementsOf(readFileSync(seedPath, "utf8")), ...seedContactsSql(seedContactCount, "'\\x'")])
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
    } finally {
      await isolated.cleanup();
    }
  });
});
