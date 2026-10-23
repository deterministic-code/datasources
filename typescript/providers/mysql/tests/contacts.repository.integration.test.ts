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
import { seedContactCount, seedContactsSql, seedIdentitySql } from "../../../datasources/src/tests/contact/seedContacts.ts";
import { ensureCompose } from "../../../ensure-compose.ts";
import { repoRoot, specVerbose } from "../../../sql-spec-runner.ts";
import { Native } from "@deterministic-code/datasources";
import { applyContacts } from "../../../contacts-spec.ts";
import { MysqlCommand } from "../src/command.ts";
import { MysqlConnection } from "../src/connection.ts";
import { MysqlDriver } from "../src/MysqlDriver.ts";
import type { Mysql2Module } from "../src/i-mysql-engine.ts";

const url = (process.env.MYSQL_URL ??=
  "mysql://ds:ds@127.0.0.1:3306/datasources");

const seedPath = join(repoRoot(), "specs/contacts/mysql.sql");

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
  it("matches enriched ContactsService to FlatToNestedMapper on the same seed", async () => {
    const isolated = await isolate(url);
    try {
      const connection = MysqlConnection.from({ DATASOURCE_URL: isolated.url });
      await connection.open();
      const command = new MysqlCommand(connection);
      await applyContacts(connection, "mysql", new MysqlDriver());
      for (const sql of [
        ...statementsOf(readFileSync(seedPath, "utf8")),
        ...seedContactsSql(seedContactCount, "X''"),
        ...seedIdentitySql("mysql"),
      ])
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
