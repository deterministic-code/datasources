import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { beforeAll, describe, expect, it } from "vitest";
import { asContactsView } from "../../../datasources/src/tests/contact/asContactsView.ts";
import { contactsParity } from "../../../datasources/src/tests/contact/contactsParity.ts";
import { expectContactsView } from "../../../datasources/src/tests/contact/expectContactsView.ts";
import { logMapped } from "../../../datasources/src/tests/contact/logMapped.ts";
import { addExtraContact } from "../../../datasources/src/tests/contact/extraContact.ts";
import { contactsServiceOf } from "../../../datasources/src/tests/contact/contactsServiceOf.ts";
import { seedContactCount, seedContactsSql } from "../../../datasources/src/tests/contact/seedContacts.ts";
import { ensureCompose } from "../../../ensure-compose.ts";
import { specVerbose } from "../../../sql-spec-runner.ts";
import {
  loadPgDriver,
  PostgresCommand,
  PostgresConnection,
} from "../src/index.ts";
import type { PgModule } from "../src/i-postgres-driver.ts";

const url = (process.env.POSTGRES_URL ??=
  "postgres://ds:ds@127.0.0.1:5432/datasources");

const schemaPath = join(
  dirname(fileURLToPath(import.meta.url)),
  "../../../datasources/src/tests/contact/postgres.sql",
);

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
  it("matches enriched ContactsService to FlatToTypeMapper on the same seed", async () => {
    const isolated = await isolate(url);
    try {
      const connection = new PostgresConnection({
        url,
        schema: isolated.schema,
        driver: await loadPgDriver(url, undefined, isolated.schema),
      });
      await connection.open();
      const command = new PostgresCommand(connection);
      for (const sql of [...statementsOf(readFileSync(schemaPath, "utf8")), ...seedContactsSql(seedContactCount, "'\\x'")])
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
    } finally {
      await isolated.cleanup();
    }
  });
});
