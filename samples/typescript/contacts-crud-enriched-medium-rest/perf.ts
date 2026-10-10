import {
  BatchCommandIBulkWriteStrategy,
  BulkCRUDRepository,
  DictionaryMapper,
  Native,
  StoredProcedureBulkWriteStrategy,
  type IBulkWriteStrategy,
  type IConnection,
} from "@datasources/core";
import { MysqlDriver } from "@datasources/mysql";
import { PostgresDriver } from "@datasources/postgres";
import { SqliteDriver } from "@datasources/sqlite";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { contacts as seedContacts } from "../../../scripts/perf-data.mjs";

type SeedStamp = {
  id: number;
  uuid: string;
  created: string;
  updated: string;
  version: number[];
};

type SeedContact = SeedStamp & {
  contactSourceId: number;
  firstName: string;
  lastName: string;
  email: string;
  notes: string;
  addresses: Array<SeedStamp & { addressTypeId: number; line1: string; city: string }>;
  phones: Array<SeedStamp & { phoneTypeId: number; number: string; label: string }>;
  settings: Record<string, string>;
};

class Address {
  id = 0;
  uuid = "";
  created = new Date();
  updated = new Date();
  version = new Uint8Array([1]);
  contactId = 0;
  addressTypeId = 1;
  line1 = "";
  city = "";
}

class Phone {
  id = 0;
  uuid = "";
  created = new Date();
  updated = new Date();
  version = new Uint8Array([1]);
  contactId = 0;
  phoneTypeId = 1;
  number = "";
  label = "";
}

class Contact {
  id = 0;
  uuid = "";
  created = new Date();
  updated = new Date();
  version = new Uint8Array([1]);
  contactSourceId = 1;
  firstName = "";
  lastName = "";
  email = "";
  notes = "";
  addresses: Address[] = [];
  phones: Phone[] = [];
  settings: Record<string, string> = {};
}

class Setting {
  contactId = 0;
  key = "";
  value = "";
}

const sample = dirname(fileURLToPath(import.meta.url));
const shared = resolve(sample, "../../contacts-crud-enriched-medium-rest-shared");
const provider = process.env.PROVIDER ?? "sqlite";
const strategyName = process.env.PERF_STRATEGY ?? "batch";
const rows = Number(process.env.PERF_ROWS ?? 1000);
const chunk = Number(process.env.PERF_CHUNK ?? 500);
const url = process.env.DATASOURCE_URL ?? "sqlite:///:memory:";

const driver = () =>
  ({ sqlite: () => new SqliteDriver(), mysql: () => new MysqlDriver(), postgres: () => new PostgresDriver() })[provider]?.() ??
  (() => {
    throw new Error(provider);
  })();

const splitDash = (sql: string) =>
  sql
    .split(/^---\s*$/m)
    .map((part) => part.trim())
    .filter(Boolean);

const splitSemi = (sql: string) =>
  sql
    .split(";")
    .map((part) => part.trim())
    .filter(Boolean);

const batched = strategyName === "batch" || (strategyName === "procedure" && provider === "sqlite");

const of = <T extends object>(create: string, update: string, remove: string): IBulkWriteStrategy<T> | undefined => {
  if (strategyName === "single") return undefined;
  if (batched) return new BatchCommandIBulkWriteStrategy<T>();
  if (strategyName === "procedure") return new StoredProcedureBulkWriteStrategy<T>(create, update, remove);
  throw new Error(strategyName);
};

const each = async <T>(items: T[], write: (batch: T[]) => Promise<unknown>) => {
  if (!batched) return write(items);
  for (let at = 0; at < items.length; at += chunk) await write(items.slice(at, at + chunk));
};

const count = async (connection: IConnection, table: string) => {
  const reader = await connection.createCommand().executeReaderNative(new Native(`SELECT COUNT(*) AS n FROM ${table}`));
  await reader.next();
  const n = reader.current?.n;
  return typeof n === "number" ? n : Number(n);
};

const connection = driver().connection({ DATASOURCE_URL: url });
await connection.open();
try {
  const command = connection.createCommand();
  for (const sql of splitSemi(readFileSync(resolve(shared, `schema-${provider}.sql`), "utf8")))
    await command.executeNative(new Native(sql));
  if (provider === "mysql" || provider === "postgres")
    for (const sql of splitDash(readFileSync(resolve(shared, `bulk-${provider}.sql`), "utf8")))
      await command.executeNative(new Native(sql));
  const now = new Date("2024-01-01T00:00:00.000Z");
  const stamp = provider === "mysql" ? "2024-01-01 00:00:00" : now.toISOString();
  const blob = provider === "mysql" ? "0x01" : provider === "postgres" ? "'\\x01'" : "x'01'";
  for (const table of ["contact_sources", "address_types", "phone_types"])
    await command.executeNative(
      new Native(
        `INSERT INTO ${table} (id, uuid, created, updated, version, name) VALUES (1, '00000000-0000-0000-0000-000000000001', '${stamp}', '${stamp}', ${blob}, 'seed')`,
      ),
    );
  const settingsRepo = new BulkCRUDRepository(connection, {
    type: Setting,
    entity: "ContactSettings",
    entityMapper: new DictionaryMapper([["ContactSettings", "contact_settings"]]),
    propertyMappings: new DictionaryMapper([["contactId", "contact_id"]]),
    ids: ["contactId", "key"],
  }, of<Setting>("bulk_create_setting", "bulk_update_setting", "bulk_delete_setting"));
  const addressesRepo = new BulkCRUDRepository(connection, {
    type: Address,
    entity: "Address",
    entityMapper: new DictionaryMapper([["Address", "addresses"]]),
    propertyMappings: new DictionaryMapper([
      ["contactId", "contact_id"],
      ["addressTypeId", "address_type_id"],
    ]),
  }, of<Address>("bulk_create_address", "bulk_update_address", "bulk_delete_address"));
  const phonesRepo = new BulkCRUDRepository(connection, {
    type: Phone,
    entity: "Phone",
    entityMapper: new DictionaryMapper([["Phone", "phones"]]),
    propertyMappings: new DictionaryMapper([
      ["contactId", "contact_id"],
      ["phoneTypeId", "phone_type_id"],
    ]),
  }, of<Phone>("bulk_create_phone", "bulk_update_phone", "bulk_delete_phone"));
  const contactsRepo = new BulkCRUDRepository(connection, {
    type: Contact,
    entity: "Contact",
    entityMapper: new DictionaryMapper([["Contact", "contacts"]]),
    propertyMappings: new DictionaryMapper([
      ["contactSourceId", "contact_source_id"],
      ["firstName", "first_name"],
      ["lastName", "last_name"],
    ]),
  }, of<Contact>("bulk_create_contact", "bulk_update_contact", "bulk_delete_contact"));
  contactsRepo.addEnriched({ destinationProperty: "addresses", sourceProperty: "id", relatedKey: "contactId", type: "list" }, addressesRepo);
  contactsRepo.addEnriched({ destinationProperty: "phones", sourceProperty: "id", relatedKey: "contactId", type: "list" }, phonesRepo);
  contactsRepo.addEnriched(
    { destinationProperty: "settings", sourceProperty: "id", relatedKey: "contactId", relatedProperty: "value", type: "dictionary" },
    settingsRepo,
  );
  const seed = (process.env.PERF_DATA
    ? JSON.parse(readFileSync(process.env.PERF_DATA, "utf8"))
    : seedContacts(rows)) as SeedContact[];
  const stamped = (row: SeedStamp) => ({
    ...row,
    created: new Date(row.created),
    updated: new Date(row.updated),
    version: Uint8Array.from(row.version),
  });
  const contacts = seed.map((row) =>
    Object.assign(new Contact(), {
      ...stamped(row),
      addresses: row.addresses.map((address) => Object.assign(new Address(), stamped(address))),
      phones: row.phones.map((phone) => Object.assign(new Phone(), stamped(phone))),
      settings: { ...row.settings },
    }),
  );
  const createStarted = performance.now();
  await each(contacts, (batch) => contactsRepo.createBulk(batch));
  const createMs = performance.now() - createStarted;
  const counts = {
    contacts: await count(connection, "contacts"),
    addresses: await count(connection, "addresses"),
    phones: await count(connection, "phones"),
    settings: await count(connection, "contact_settings"),
  };
  const each = {
    addresses: contacts[0]?.addresses.length ?? 0,
    phones: contacts[0]?.phones.length ?? 0,
    settings: Object.keys(contacts[0]?.settings ?? {}).length,
  };
  if (
    counts.contacts !== contacts.length ||
    counts.addresses !== contacts.length * each.addresses ||
    counts.phones !== contacts.length * each.phones ||
    counts.settings !== contacts.length * each.settings
  )
    throw new Error(`create count ${JSON.stringify(counts)}`);
  for (const contact of contacts) {
    contact.notes = "updated";
    for (const address of contact.addresses) address.city = "City";
    for (const phone of contact.phones) phone.label = "w";
    for (const key of Object.keys(contact.settings)) contact.settings[key] = "dark";
  }
  const updateStarted = performance.now();
  await each(contacts, (batch) => contactsRepo.updateBulk(batch));
  const updateMs = performance.now() - updateStarted;
  process.stdout.write(
    `${JSON.stringify({
      sample: "contacts-crud-enriched-medium-rest",
      language: "typescript",
      provider,
      strategy: strategyName,
      rows: contacts.length,
      chunk: batched ? Math.min(chunk, contacts.length) : contacts.length,
      createMs,
      updateMs,
      createPerSec: contacts.length / (createMs / 1000),
      updatePerSec: contacts.length / (updateMs / 1000),
      counts,
    })}\n`,
  );
} finally {
  await connection.close().catch(() => undefined);
}
