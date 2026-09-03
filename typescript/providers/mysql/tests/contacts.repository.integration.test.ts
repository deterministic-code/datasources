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
  loadMysql2Driver,
  MysqlCommand,
  MysqlConnection,
} from "../src/index.ts";
import type { Mysql2Module } from "../src/i-mysql-driver.ts";

const url = (process.env.MYSQL_URL ??=
  "mysql://ds:ds@127.0.0.1:3306/datasources");

const schemaPath = join(
  dirname(fileURLToPath(import.meta.url)),
  "../../../datasources/src/tests/contact/mysql.sql",
);

const statementsOf = (sql: string) =>
  sql
    .split(";")
    .map((part) => part.trim())
    .filter(Boolean);

beforeAll(() => ensureCompose("mysql"), 120_000);

const replaceDatabase = (base: string, name: string): string => {
  const parsed = new URL(base);
  parsed.pathname = `/${name}`;
  return parsed.toString();
};

const isolate = async (base: string) => {
  const mysql = (await import("mysql2/promise")) as Mysql2Module;
  const name = `ds_${crypto.randomUUID().replaceAll("-", "").slice(0, 16)}`;
  const admin = await mysql.createConnection({
    uri: base,
    namedPlaceholders: true,
  });
  await admin.query(`CREATE DATABASE \`${name}\``);
  await admin.end();
  return {
    url: replaceDatabase(base, name),
    cleanup: async () => {
      const drop = await mysql.createConnection({
        uri: base,
        namedPlaceholders: true,
      });
      await drop.query(`DROP DATABASE \`${name}\``);
      await drop.end();
    },
  };
};

describe("contacts readonly repository", () => {
  it("matches enriched ContactsService to FlatToTypeMapper on the same seed", async () => {
    const isolated = await isolate(url);
    try {
      const connection = new MysqlConnection({
        url: isolated.url,
        driver: await loadMysql2Driver(isolated.url),
      });
      await connection.open();
      const command = new MysqlCommand(connection);
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
    } finally {
      await isolated.cleanup();
    }
  });
});
