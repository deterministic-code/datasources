import {
  BatchCommandIBulkWriteStrategy,
  BulkCRUDRepository,
  CRUDRepository,
  DictionaryMapper,
  FlatToNestedMapper,
  HashsetOptimizedReadonlyEnrichmentStrategy,
  Identifier,
  Native,
  Operator,
  SelectStatement,
  StoredProcedureBulkWriteStrategy,
  WhereText,
  collectAsync,
  type IBulkWriteStrategy,
  type IConnection,
  type IWhere,
  type TypeMap,
} from "@deterministic-code/datasources";
import { MysqlDriver } from "@deterministic-code/datasources-mysql";
import { PostgresDriver } from "@deterministic-code/datasources-postgres";
import { SqliteDriver } from "@deterministic-code/datasources-sqlite";
import { execFileSync } from "node:child_process";
import { mkdtempSync, readFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { contacts as seedContacts } from "../../../scripts/perf-data.mjs";
import { migrateUp } from "../../../scripts/migraters.mjs";

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
const spec = resolve(sample, "../../contacts-crud-enriched-medium-rest-shared/deterministic");
const provider = process.env.PROVIDER ?? "sqlite";
const strategyName = process.env.PERF_STRATEGY ?? "batch";
const rows = Number(process.env.PERF_ROWS ?? 1000);
const chunk = Number(process.env.PERF_CHUNK ?? 500);
const url = process.env.DATASOURCE_URL ?? `sqlite:///${resolve(sample, "perf.sqlite")}`;
const reading = process.env.PERF_PHASE === "read";

const driver = () =>
  ({ sqlite: () => new SqliteDriver(), mysql: () => new MysqlDriver(), postgres: () => new PostgresDriver() })[provider]?.() ??
  (() => {
    throw new Error(provider);
  })();

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

const sqlRoot = mkdtempSync(join(tmpdir(), "contacts-perf-sql-"));
execFileSync(
  process.execPath,
  ["--import", "tsx", resolve(sample, "../../../cli/src/main.ts"), "migrations", "create", "--input", spec, "--migrations-root", join(sqlRoot, "sql")],
  { encoding: "utf8", cwd: resolve(sample, "../../../cli") },
);
migrateUp({ provider, connection: url, migrationsRoot: join(sqlRoot, "sql") });
const connection = driver().connection({ DATASOURCE_URL: url });
await connection.open();
try {
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
  let createMs = 0;
  let updateMs = 0;
  let total = rows;
  let relatedCount = Number(process.env.PERF_RELATED ?? 10);
  let counts = { contacts: 0, addresses: 0, phones: 0, settings: 0 };
  if (!reading) {
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
    createMs = performance.now() - createStarted;
    counts = {
      contacts: await count(connection, "contacts"),
      addresses: await count(connection, "addresses"),
      phones: await count(connection, "phones"),
      settings: await count(connection, "contact_settings"),
    };
    const related = {
      addresses: contacts[0]?.addresses.length ?? 0,
      phones: contacts[0]?.phones.length ?? 0,
      settings: Object.keys(contacts[0]?.settings ?? {}).length,
    };
    if (
      counts.contacts !== contacts.length ||
      counts.addresses !== contacts.length * related.addresses ||
      counts.phones !== contacts.length * related.phones ||
      counts.settings !== contacts.length * related.settings
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
    updateMs = performance.now() - updateStarted;
    total = contacts.length;
    relatedCount = related.addresses;
  } else {
    counts = {
      contacts: await count(connection, "contacts"),
      addresses: await count(connection, "addresses"),
      phones: await count(connection, "phones"),
      settings: await count(connection, "contact_settings"),
    };
    total = counts.contacts;
    if (total === 0) throw new Error("read store is empty; run npm run test:perf first");
  }
  const hashsetRepo = new BulkCRUDRepository(connection, {
    type: Contact,
    entity: "Contact",
    entityMapper: new DictionaryMapper([["Contact", "contacts"]]),
    propertyMappings: new DictionaryMapper([
      ["contactSourceId", "contact_source_id"],
      ["firstName", "first_name"],
      ["lastName", "last_name"],
    ]),
  });
  const hashset = new HashsetOptimizedReadonlyEnrichmentStrategy<Contact>();
  hashsetRepo.addEnriched({ destinationProperty: "addresses", sourceProperty: "id", relatedKey: "contactId", type: "list" }, addressesRepo, hashset);
  hashsetRepo.addEnriched({ destinationProperty: "phones", sourceProperty: "id", relatedKey: "contactId", type: "list" }, phonesRepo, hashset);
  hashsetRepo.addEnriched(
    { destinationProperty: "settings", sourceProperty: "id", relatedKey: "contactId", relatedProperty: "value", type: "dictionary" },
    settingsRepo,
    hashset,
  );
  const joinRepo = new CRUDRepository(connection, {
    entity: "Contact",
    entityMapper: new DictionaryMapper([
      ["Contact", "contacts"],
      ["Address", "addresses"],
      ["Phone", "phones"],
      ["ContactSettings", "contact_settings"],
    ]),
    propertyMappings: new DictionaryMapper([
      ["firstName", "first_name"],
      ["contactId", "contact_id"],
    ]),
  });
  const reads = reading
    ? await profileReads(contactsRepo, hashsetRepo, joinRepo, total, relatedCount)
    : undefined;
  process.stdout.write(
    `${JSON.stringify({
      sample: "contacts-crud-enriched-medium-rest",
      language: "typescript",
      provider,
      strategy: strategyName,
      rows: total,
      chunk: batched ? Math.min(chunk, total) : total,
      createMs,
      updateMs,
      createPerSec: createMs === 0 ? 0 : total / (createMs / 1000),
      updatePerSec: updateMs === 0 ? 0 : total / (updateMs / 1000),
      counts,
      reads,
    })}\n`,
  );
} finally {
  await connection.close().catch(() => undefined);
}

type FlatContact = {
  id: number;
  firstName: string;
  addresses: { id: number }[];
  phones: { id: number }[];
  settings: { key: string; value: string }[];
};


function next(state: { n: number }) {
  state.n = (Math.imul(1664525, state.n) + 1013904223) >>> 0;
  return state.n / 0x100000000;
}

function sampleIds(state: { n: number }, total: number, items: number, iter: number) {
  const ids = Array.from({ length: total }, (_, i) => i + 1);
  for (let i = 0; i < items; i++) {
    const swap = i + Math.floor(next(state) * (total - i));
    [ids[i], ids[swap]] = [ids[swap], ids[i]];
  }
  return { ids: ids.slice(0, items), lo: items >= total ? 0 : (iter * items) % (total - items + 1) };
}

function median(values: number[]) {
  const sorted = [...values].sort((left, right) => left - right);
  const mid = Math.floor(sorted.length / 2);
  return sorted.length % 2 === 1 ? sorted[mid] : (sorted[mid - 1] + sorted[mid]) / 2;
}

async function time(work: () => Promise<void>) {
  const started = performance.now();
  await work();
  return performance.now() - started;
}

function joinWhere(shape: string, picked: { ids: number[]; lo: number }): IWhere {
  return ({
    ids: () => new WhereText(`contacts.id IN (${picked.ids.join(",")})`),
    where: () => new WhereText(`contacts.id > ${picked.lo} AND contacts.id <= ${picked.lo + picked.ids.length}`),
    firstName: () => new WhereText(`contacts.first_name IN (${picked.ids.map((id) => `'First${id}'`).join(",")})`),
  }[shape] ?? (() => { throw new Error(shape); }))();
}

function joinSelect(where: IWhere) {
  return new SelectStatement({
    table: "Contact",
    fields: [
      "Contact.id AS id",
      "Contact.firstName AS firstName",
      "Address.id AS addressId",
      "Phone.id AS phoneId",
      "ContactSettings.key AS settingKey",
      "ContactSettings.value AS settingValue",
    ],
    joins: [
      { kind: "left", table: "Address", on: new Operator.Equals(new Identifier("Contact.id"), new Identifier("Address.contactId")) },
      { kind: "left", table: "Phone", on: new Operator.Equals(new Identifier("Contact.id"), new Identifier("Phone.contactId")) },
      { kind: "left", table: "ContactSettings", on: new Operator.Equals(new Identifier("Contact.id"), new Identifier("ContactSettings.contactId")) },
    ],
    where,
  });
}

function checkEnriched(loaded: Contact[], expected: number, relatedCount: number, check: boolean) {
  if (!check) return;
  if (loaded.length !== expected) throw new Error(`read count ${loaded.length}`);
  if (loaded.some((contact) => contact.addresses.length !== relatedCount || contact.phones.length !== relatedCount || Object.keys(contact.settings).length !== relatedCount))
    throw new Error("read related");
}

function checkMapped(loaded: FlatContact[], expected: number, relatedCount: number, check: boolean) {
  if (!check) return;
  if (loaded.length !== expected) throw new Error(`flat count ${loaded.length}`);
  if (loaded.some((row) => row.addresses.length !== relatedCount || row.phones.length !== relatedCount || row.settings.length !== relatedCount))
    throw new Error("flat related");
}

function contactView(): TypeMap<FlatContact> {
  return {
  key: "id",
  fields: {
    id: "id",
    firstName: "firstName",
    addresses: { key: "addressId", fields: { id: "addressId" } },
    phones: { key: "phoneId", fields: { id: "phoneId" } },
    settings: { key: "settingKey", fields: { key: "settingKey", value: "settingValue" } },
  },
  };
}

async function profileReads(
  contactsRepo: BulkCRUDRepository<Contact>,
  hashsetRepo: BulkCRUDRepository<Contact>,
  joinRepo: CRUDRepository,
  total: number,
  relatedCount: number,
) {
  const items = Math.min(Number(process.env.PERF_READ_ITEMS ?? 100), total);
  const iters = Number(process.env.PERF_READ_ITERS ?? 5);
  const state = { n: 42 };
  const mapper = new FlatToNestedMapper(contactView());
  const times: Record<string, Record<string, number[]>> = {
    default: { ids: [], where: [], firstName: [] },
    hashset: { ids: [], where: [], firstName: [] },
    flat: { ids: [], where: [], firstName: [] },
  };
  const run = async (picked: { ids: number[]; lo: number }, check: boolean) => {
    const shapes: Record<string, IWhere> = {
      ids: new WhereText(`id IN (${picked.ids.join(",")})`),
      where: new WhereText(`id > ${picked.lo} AND id <= ${picked.lo + picked.ids.length}`),
      firstName: new WhereText(`first_name IN (${picked.ids.map((id) => `'First${id}'`).join(",")})`),
    };
    const measured: Record<string, Record<string, number>> = { default: {}, hashset: {}, flat: {} };
    for (const [shape, where] of Object.entries(shapes)) {
      measured.default[shape] = await time(async () => checkEnriched(await collectAsync(contactsRepo.read(where)), picked.ids.length, relatedCount, check));
      measured.hashset[shape] = await time(async () => checkEnriched(await collectAsync(hashsetRepo.read(where)), picked.ids.length, relatedCount, check));
      measured.flat[shape] = await time(async () => checkMapped(mapper.map(await collectAsync(joinRepo.read(joinSelect(joinWhere(shape, picked))))), picked.ids.length, relatedCount, check));
    }
    return measured;
  };
  await run(sampleIds(state, total, items, 0), false);
  for (let iter = 0; iter < iters; iter++) {
    const measured = await run(sampleIds(state, total, items, iter), iter === 0);
    for (const [mode, shapes] of Object.entries(measured))
      for (const [shape, ms] of Object.entries(shapes)) times[mode][shape].push(ms);
  }
  return Object.fromEntries(
    Object.entries(times).map(([mode, shapes]) => [
      mode,
      Object.fromEntries(
        Object.entries(shapes).map(([shape, values]) => [
          shape,
          { ms: median(values), perSec: items / (median(values) / 1000) },
        ]),
      ),
    ]),
  );
}
