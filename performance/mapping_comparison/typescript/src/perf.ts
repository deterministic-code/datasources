import {
  BatchCommandIBulkWriteStrategy,
  BulkCRUDRepository,
  ChangeCasingMapper,
  Datasource,
  DatasourceFactory,
  DatasourceUrlParser,
  DictionaryMapper,
  Native,
  ReadonlyRepository,
  Schema,
  StoredProcedureBulkWriteStrategy,
  collectAsync,
  type IBulkWriteStrategy,
  type IConnection,
  type Cells,
} from "@deterministic-code/datasources";
import { unlinkSync } from "node:fs";
import { spawnSync } from "node:child_process";
import "../generated/repositories/repositoryRegistry.ts";
import { ContactsRepository } from "../generated/repositories/contactsRepository.ts";
import { Address, address } from "../generated/types/view/address.ts";
import { Contact, contact } from "../generated/types/view/contact.ts";
import { Email, email } from "../generated/types/view/email.ts";

const processStarted = performance.now();
const provider = process.env.PROVIDER ?? "sqlite";
const sourceUrl = process.env.DATASOURCE_URL ?? "sqlite:///./app.sqlite";
const rows = Number(process.env.PERF_ROWS ?? 25000);
const chunk = Number(process.env.PERF_CHUNK ?? 500);
const readIters = Number(process.env.PERF_READ_ITERS ?? 3);
Datasource.default;

const of = <T extends object>(create: string, update: string, remove: string): IBulkWriteStrategy<T> => {
  if (provider === "sqlite") return new BatchCommandIBulkWriteStrategy<T>();
  return new StoredProcedureBulkWriteStrategy<T>(create, update, remove);
};

const each = async <T>(items: T[], write: (batch: T[]) => Promise<unknown>) => {
  for (let at = 0; at < items.length; at += chunk) await write(items.slice(at, at + chunk));
};

const median = (values: number[]) => {
  const sorted = [...values].sort((left, right) => left - right);
  const mid = Math.floor(sorted.length / 2);
  return sorted.length % 2 === 1 ? sorted[mid]! : (sorted[mid - 1]! + sorted[mid]!) / 2;
};

const seedContacts = (total: number) => {
  const contacts: Contact[] = [];
  for (let i = 1; i <= total; i++) {
    contacts.push(
      contact({
        name: `Contact ${i}`,
        givenName: `Given${i}`,
        familyName: `Family${i}`,
        genderTypeId: 1 + (i % 3),
        isStarred: i % 2 === 0,
        isFavorite: i % 3 === 0,
        isDeleted: i % 5 === 0,
        addresses: [
          address({ line1: `${i} Line 1`, city: `City${i % 50}`, countryCode: "US", isPrimary: true, latitude: null, longitude: null }),
          address({ line1: `${i} Line 2`, city: `City${(i + 1) % 50}`, countryCode: "US", isPrimary: false, latitude: null, longitude: null }),
        ],
        emails: [
          email({ address: `c${i}.e1@example.com`, displayName: "Email 1", emailTypeId: 1, isPrimary: true }),
          email({ address: `c${i}.e2@example.com`, displayName: "Email 2", emailTypeId: 2, isPrimary: false }),
        ],
      }),
    );
  }
  return contacts;
};

const uuid = /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i;
const mappedDate = (value: Date) => value instanceof Date && !Number.isNaN(value.getTime()) && value.getUTCFullYear() >= 2020;

const check = (loaded: Contact[], label: string) => {
  if (loaded.length !== rows) throw new Error(`${label} count ${loaded.length}`);
  for (const row of loaded) {
    if (row.addresses.length !== 2 || row.emails.length !== 2) throw new Error(`${label} related ${row.givenName}`);
    if (!uuid.test(row.uuid) || row.addresses.some((item) => !uuid.test(item.uuid)) || row.emails.some((item) => !uuid.test(item.uuid)))
      throw new Error(`${label} uuid ${row.givenName}`);
    if (!mappedDate(row.created) || !mappedDate(row.updated)
      || row.addresses.some((item) => !mappedDate(item.created) || !mappedDate(item.updated))
      || row.emails.some((item) => !mappedDate(item.created) || !mappedDate(item.updated)))
      throw new Error(`${label} datetime ${row.givenName}`);
    if (!row.givenName.startsWith("Given")) throw new Error(`${label} givenName ${row.givenName}`);
    const n = Number(row.givenName.slice("Given".length));
    if (!Number.isInteger(n) || row.isStarred !== (n % 2 === 0) || row.isFavorite !== (n % 3 === 0) || row.isDeleted !== (n % 5 === 0))
      throw new Error(`${label} boolean ${row.givenName}`);
    if (!row.addresses[0]!.isPrimary || row.addresses[1]!.isPrimary || !row.emails[0]!.isPrimary || row.emails[1]!.isPrimary)
      throw new Error(`${label} isPrimary ${row.givenName}`);
  }
};

const exec = (connection: IConnection, sql: string) => connection.createCommand().executeNative(new Native(sql));

const rowsOf = async (connection: IConnection, sql: string) => {
  const reader = await connection.createCommand().executeReaderNative(new Native(sql));
  const found: Cells[] = [];
  while (await reader.next()) if (reader.current) found.push(reader.current);
  return found;
};

const cell = (row: Cells, name: string) => {
  for (const [key, value] of Object.entries(row))
    if (key.toLowerCase() === name.toLowerCase()) return value == null ? "" : String(value);
  return "";
};

const withDatasource = (url: string, name: string) => {
  const parsed = DatasourceUrlParser.parse(url);
  if (parsed?.provider === "sqlite") {
    const file = parsed.datasource ?? "app.sqlite";
    const slash = file.lastIndexOf("/");
    const dir = slash < 0 ? "." : file.slice(0, slash);
    return `sqlite:///${dir}/${name}.sqlite`;
  }
  const slash = url.lastIndexOf("/");
  const query = url.indexOf("?", slash < 0 ? 0 : slash);
  if (slash < 0) return url;
  return query < 0 ? `${url.slice(0, slash + 1)}${name}` : `${url.slice(0, slash + 1)}${name}${url.slice(query)}`;
};

const tryUnlink = (path: string) => {
  try {
    unlinkSync(path);
  } catch {
    /* missing */
  }
};

const admin = () =>
  DatasourceFactory.create({
    PROVIDER: provider,
    DATASOURCE_URL: provider === "postgres" ? withDatasource(sourceUrl, "postgres") : sourceUrl,
  });

const npx = (...args: string[]) => {
  const result = spawnSync("npx", args, { encoding: "utf8" });
  if (result.status !== 0) throw new Error(result.stderr || result.stdout || `npx ${args.join(" ")}`);
};

const migrateUp = (url: string) => {
  npx("migrate-setup", "--provider", provider, "--connection", url);
  npx("migrate-up", "--provider", provider, "--connection", url, "--migrations-root", "sql");
};

const isolate = async (name: string) => {
  const catalog = `mapping_${name}`;
  const url = withDatasource(sourceUrl, catalog);
  if (provider === "sqlite") {
    const file = DatasourceUrlParser.parse(url)?.datasource;
    if (file) {
      tryUnlink(file);
      tryUnlink(`${file}-wal`);
      tryUnlink(`${file}-shm`);
    }
    migrateUp(url);
    return url;
  }
  const ds = admin();
  const connection = await ds.createConnection();
  await connection.open();
  try {
    await exec(connection, provider === "postgres" ? `DROP DATABASE IF EXISTS ${catalog} WITH (FORCE)` : `DROP DATABASE IF EXISTS \`${catalog}\``);
    await exec(connection, provider === "postgres" ? `CREATE DATABASE ${catalog}` : `CREATE DATABASE \`${catalog}\``);
  } finally {
    await connection.close();
  }
  migrateUp(url);
  return url;
};

const dropIsolated = async (name: string) => {
  const catalog = `mapping_${name}`;
  if (provider === "sqlite") {
    const file = DatasourceUrlParser.parse(withDatasource(sourceUrl, catalog))?.datasource;
    if (!file) return;
    tryUnlink(file);
    tryUnlink(`${file}-wal`);
    tryUnlink(`${file}-shm`);
    return;
  }
  const ds = admin();
  const connection = await ds.createConnection();
  await connection.open();
  try {
    await exec(connection, provider === "postgres" ? `DROP DATABASE IF EXISTS ${catalog} WITH (FORCE)` : `DROP DATABASE IF EXISTS \`${catalog}\``);
  } finally {
    await connection.close();
  }
};

const countContacts = async (connection: IConnection) => {
  const found = await rowsOf(connection, "SELECT COUNT(*) AS n FROM contacts");
  const n = Number(found[0] ? cell(found[0], "n") : 0);
  if (n !== rows) throw new Error(`create count contacts=${n}`);
};

const contactsBulk = (connection: IConnection) => {
  const sparseContact = new DictionaryMapper([
    ["givenName", "given_name"],
    ["additionalName", "additional_name"],
    ["familyName", "family_name"],
    ["nickName", "nick_name"],
    ["jobTitle", "job_title"],
    ["isStarred", "is_starred"],
    ["isFavorite", "is_favorite"],
    ["isDeleted", "is_deleted"],
    ["genderTypeId", "gender_type_id"],
  ]);
  const sparseAddress = new DictionaryMapper([
    ["contactId", "contact_id"],
    ["displayName", "display_name"],
    ["line1", "line_1"],
    ["line2", "line_2"],
    ["line3", "line_3"],
    ["stateProvince", "state_province"],
    ["postalCode", "postal_code"],
    ["countryCode", "country_code"],
    ["formattedAddress", "formatted_address"],
    ["isPrimary", "is_primary"],
  ]);
  const sparseEmail = new DictionaryMapper([
    ["contactId", "contact_id"],
    ["displayName", "display_name"],
    ["isPrimary", "is_primary"],
    ["emailTypeId", "email_type_id"],
  ]);
  const contactColumns = [
    "id", "uuid", "created", "updated", "version", "name", "givenName", "additionalName", "pronunciation",
    "familyName", "nickName", "jobTitle", "organization", "isStarred", "isFavorite", "isDeleted", "genderTypeId",
  ];
  const addressColumns = [
    "id", "uuid", "created", "updated", "version", "contactId", "displayName", "line1", "line2", "line3",
    "neighborhood", "district", "city", "stateProvince", "postalCode", "countryCode", "formattedAddress",
    "latitude", "longitude", "isPrimary",
  ];
  const emailColumns = [
    "id", "uuid", "created", "updated", "version", "emailTypeId", "contactId", "address", "displayName", "isPrimary",
  ];
  const contactSchema = new Schema({
    id: { type: "integer", nullable: false, hasDefault: false, isId: true },
    uuid: { type: "uuid", nullable: false, hasDefault: true },
    created: { type: "datetime", nullable: false, hasDefault: true },
    updated: { type: "datetime", nullable: false, hasDefault: true },
    version: { type: "binary", nullable: false, hasDefault: true },
    name: { type: "string", nullable: false, hasDefault: false },
    givenName: { type: "string", nullable: false, hasDefault: false },
    additionalName: { type: "string", nullable: false, hasDefault: true },
    pronunciation: { type: "string", nullable: false, hasDefault: true },
    familyName: { type: "string", nullable: false, hasDefault: true },
    nickName: { type: "string", nullable: false, hasDefault: true },
    jobTitle: { type: "string", nullable: false, hasDefault: true },
    organization: { type: "string", nullable: false, hasDefault: true },
    isStarred: { type: "boolean", nullable: false, hasDefault: false },
    isFavorite: { type: "boolean", nullable: false, hasDefault: false },
    isDeleted: { type: "boolean", nullable: false, hasDefault: false },
    genderTypeId: { type: "integer", nullable: false, hasDefault: false },
  });
  const addressSchema = new Schema({
    id: { type: "integer", nullable: false, hasDefault: false, isId: true },
    uuid: { type: "uuid", nullable: false, hasDefault: true },
    created: { type: "datetime", nullable: false, hasDefault: true },
    updated: { type: "datetime", nullable: false, hasDefault: true },
    version: { type: "binary", nullable: false, hasDefault: true },
    contactId: { type: "integer", nullable: false, hasDefault: false },
    displayName: { type: "string", nullable: false, hasDefault: true },
    line1: { type: "string", nullable: false, hasDefault: false },
    line2: { type: "string", nullable: false, hasDefault: true },
    line3: { type: "string", nullable: false, hasDefault: true },
    neighborhood: { type: "string", nullable: false, hasDefault: true },
    district: { type: "string", nullable: false, hasDefault: true },
    city: { type: "string", nullable: false, hasDefault: false },
    stateProvince: { type: "string", nullable: false, hasDefault: true },
    postalCode: { type: "string", nullable: false, hasDefault: true },
    countryCode: { type: "string", nullable: false, hasDefault: false },
    formattedAddress: { type: "string", nullable: false, hasDefault: true },
    latitude: { type: "decimal", nullable: true, hasDefault: false },
    longitude: { type: "decimal", nullable: true, hasDefault: false },
    isPrimary: { type: "boolean", nullable: false, hasDefault: true },
  });
  const emailSchema = new Schema({
    id: { type: "integer", nullable: false, hasDefault: false, isId: true },
    uuid: { type: "uuid", nullable: false, hasDefault: true },
    created: { type: "datetime", nullable: false, hasDefault: true },
    updated: { type: "datetime", nullable: false, hasDefault: true },
    version: { type: "binary", nullable: false, hasDefault: true },
    emailTypeId: { type: "integer", nullable: false, hasDefault: false },
    contactId: { type: "integer", nullable: false, hasDefault: false },
    address: { type: "string", nullable: false, hasDefault: false },
    displayName: { type: "string", nullable: false, hasDefault: true },
    isPrimary: { type: "boolean", nullable: false, hasDefault: false },
  });
  const addressesWrite = new BulkCRUDRepository(
    connection,
    {
      type: Address,
      entity: "Address",
      entityMapper: new DictionaryMapper([["Address", "addresses"]]),
      propertyMappings: sparseAddress,
      ids: ["id"],
      createProperties: [...addressColumns],
      updateProperties: [...addressColumns],
      readProperties: [...addressColumns],
      schema: addressSchema,
    },
    of<Address>("bulk_create_addresses", "bulk_update_addresses", "bulk_delete_addresses"),
  );
  const emailsWrite = new BulkCRUDRepository(
    connection,
    {
      type: Email,
      entity: "Email",
      entityMapper: new DictionaryMapper([["Email", "emails"]]),
      propertyMappings: sparseEmail,
      ids: ["id"],
      createProperties: [...emailColumns],
      updateProperties: [...emailColumns],
      readProperties: [...emailColumns],
      schema: emailSchema,
    },
    of<Email>("bulk_create_emails", "bulk_update_emails", "bulk_delete_emails"),
  );
  const bulk = new BulkCRUDRepository(
    connection,
    {
      type: Contact,
      entity: "Contact",
      entityMapper: new DictionaryMapper([["Contact", "contacts"]]),
      propertyMappings: sparseContact,
      ids: ["id"],
      createProperties: [...contactColumns],
      updateProperties: [...contactColumns],
      readProperties: [...contactColumns],
      schema: contactSchema,
    },
    of<Contact>("bulk_create_contacts", "bulk_update_contacts", "bulk_delete_contacts"),
  );
  bulk.addEnriched({ destinationProperty: "addresses", sourceProperty: "id", relatedKey: "contactId", type: "list" }, addressesWrite);
  bulk.addEnriched({ destinationProperty: "emails", sourceProperty: "id", relatedKey: "contactId", type: "list" }, emailsWrite);
  return bulk;
};

const performanceTestDefaultRead = async () => {
  const url = await isolate("default_read");
  await using ds = Datasource.create({ PROVIDER: provider, DATASOURCE_URL: url });
  const connection = await ds.createConnection();
  await connection.open();
  let seedMs = 0;
  try {
    const bulk = contactsBulk(connection);
    const setup = async () => {
      const seeded = seedContacts(rows);
      const started = performance.now();
      await each(seeded, (batch) => bulk.createBulk(batch));
      seedMs = performance.now() - started;
      await countContacts(connection);
    };
    const performanceDefaultRead = async () => {
      const repo = ds.createRepository<Contact>("Contact", connection);
      let loaded = await collectAsync(repo.read());
      check(loaded, "default");
      const times: number[] = [];
      for (let iter = 0; iter < readIters; iter++) {
        const started = performance.now();
        loaded = await collectAsync(repo.read());
        check(loaded, "default");
        times.push(performance.now() - started);
      }
      const ms = median(times);
      return {
        ms,
        perSec: rows / (ms / 1000),
        counts: { contacts: loaded.length, addresses: loaded.reduce((sum, row) => sum + row.addresses.length, 0), emails: loaded.reduce((sum, row) => sum + row.emails.length, 0) },
      };
    };
    const teardown = async () => {
      await connection.close();
      await dropIsolated("default_read");
    };
    await setup();
    const read = await performanceDefaultRead();
    await teardown();
    return { seedMs, readMs: read.ms, readPerSec: read.perSec, counts: read.counts };
  } finally {
    if (connection.isOpen) await connection.close().catch(() => undefined);
  }
};

const performanceTestDictionaryRead = async () => {
  const url = await isolate("dictionary_read");
  await using ds = Datasource.create({ PROVIDER: provider, DATASOURCE_URL: url });
  const connection = await ds.createConnection();
  await connection.open();
  let seedMs = 0;
  try {
    const snake = new ChangeCasingMapper("camelCase", "snakeCase");
    const props = (names: string[]) => new DictionaryMapper(names.map((name) => [name, snake.map(name)]));
    const contactColumns = [
      "id", "uuid", "created", "updated", "version", "name", "givenName", "additionalName", "pronunciation",
      "familyName", "nickName", "jobTitle", "organization", "isStarred", "isFavorite", "isDeleted", "genderTypeId",
    ];
    const addressColumns = [
      "id", "uuid", "created", "updated", "version", "contactId", "displayName", "line1", "line2", "line3",
      "neighborhood", "district", "city", "stateProvince", "postalCode", "countryCode", "formattedAddress",
      "latitude", "longitude", "isPrimary",
    ];
    const emailColumns = [
      "id", "uuid", "created", "updated", "version", "emailTypeId", "contactId", "address", "displayName", "isPrimary",
    ];
    const contactSchema = new Schema({
      id: { type: "integer", nullable: false, hasDefault: false, isId: true },
      uuid: { type: "uuid", nullable: false, hasDefault: true },
      created: { type: "datetime", nullable: false, hasDefault: true },
      updated: { type: "datetime", nullable: false, hasDefault: true },
      version: { type: "binary", nullable: false, hasDefault: true },
      name: { type: "string", nullable: false, hasDefault: false },
      givenName: { type: "string", nullable: false, hasDefault: false },
      additionalName: { type: "string", nullable: false, hasDefault: true },
      pronunciation: { type: "string", nullable: false, hasDefault: true },
      familyName: { type: "string", nullable: false, hasDefault: true },
      nickName: { type: "string", nullable: false, hasDefault: true },
      jobTitle: { type: "string", nullable: false, hasDefault: true },
      organization: { type: "string", nullable: false, hasDefault: true },
      isStarred: { type: "boolean", nullable: false, hasDefault: false },
      isFavorite: { type: "boolean", nullable: false, hasDefault: false },
      isDeleted: { type: "boolean", nullable: false, hasDefault: false },
      genderTypeId: { type: "integer", nullable: false, hasDefault: false },
    });
    const addressSchema = new Schema({
      id: { type: "integer", nullable: false, hasDefault: false, isId: true },
      uuid: { type: "uuid", nullable: false, hasDefault: true },
      created: { type: "datetime", nullable: false, hasDefault: true },
      updated: { type: "datetime", nullable: false, hasDefault: true },
      version: { type: "binary", nullable: false, hasDefault: true },
      contactId: { type: "integer", nullable: false, hasDefault: false },
      displayName: { type: "string", nullable: false, hasDefault: true },
      line1: { type: "string", nullable: false, hasDefault: false },
      line2: { type: "string", nullable: false, hasDefault: true },
      line3: { type: "string", nullable: false, hasDefault: true },
      neighborhood: { type: "string", nullable: false, hasDefault: true },
      district: { type: "string", nullable: false, hasDefault: true },
      city: { type: "string", nullable: false, hasDefault: false },
      stateProvince: { type: "string", nullable: false, hasDefault: true },
      postalCode: { type: "string", nullable: false, hasDefault: true },
      countryCode: { type: "string", nullable: false, hasDefault: false },
      formattedAddress: { type: "string", nullable: false, hasDefault: true },
      latitude: { type: "decimal", nullable: true, hasDefault: false },
      longitude: { type: "decimal", nullable: true, hasDefault: false },
      isPrimary: { type: "boolean", nullable: false, hasDefault: true },
    });
    const emailSchema = new Schema({
      id: { type: "integer", nullable: false, hasDefault: false, isId: true },
      uuid: { type: "uuid", nullable: false, hasDefault: true },
      created: { type: "datetime", nullable: false, hasDefault: true },
      updated: { type: "datetime", nullable: false, hasDefault: true },
      version: { type: "binary", nullable: false, hasDefault: true },
      emailTypeId: { type: "integer", nullable: false, hasDefault: false },
      contactId: { type: "integer", nullable: false, hasDefault: false },
      address: { type: "string", nullable: false, hasDefault: false },
      displayName: { type: "string", nullable: false, hasDefault: true },
      isPrimary: { type: "boolean", nullable: false, hasDefault: false },
    });
    const bulk = contactsBulk(connection);
    const addresses = new ReadonlyRepository(connection, {
      type: Address,
      entity: "Address",
      entityMapper: new DictionaryMapper([["Address", "addresses"]]),
      propertyMappings: props([...addressColumns]),
      readProperties: [...addressColumns],
      schema: addressSchema,
    });
    const emails = new ReadonlyRepository(connection, {
      type: Email,
      entity: "Email",
      entityMapper: new DictionaryMapper([["Email", "emails"]]),
      propertyMappings: props([...emailColumns]),
      readProperties: [...emailColumns],
      schema: emailSchema,
    });
    const contacts = new ReadonlyRepository(connection, {
      type: Contact,
      entity: "Contact",
      entityMapper: new DictionaryMapper([["Contact", "contacts"]]),
      propertyMappings: props([...contactColumns]),
      readProperties: [...contactColumns],
      schema: contactSchema,
    });
    contacts.addEnriched({ destinationProperty: "addresses", sourceProperty: "id", relatedKey: "contactId", type: "list" }, addresses);
    contacts.addEnriched({ destinationProperty: "emails", sourceProperty: "id", relatedKey: "contactId", type: "list" }, emails);
    const setup = async () => {
      const seeded = seedContacts(rows);
      const started = performance.now();
      await each(seeded, (batch) => bulk.createBulk(batch));
      seedMs = performance.now() - started;
      await countContacts(connection);
    };
    const performanceDictionaryRead = async () => {
      let loaded = await collectAsync(contacts.read());
      check(loaded, "dictionary");
      const times: number[] = [];
      for (let iter = 0; iter < readIters; iter++) {
        const started = performance.now();
        loaded = await collectAsync(contacts.read());
        check(loaded, "dictionary");
        times.push(performance.now() - started);
      }
      const ms = median(times);
      return {
        ms,
        perSec: rows / (ms / 1000),
        counts: { contacts: loaded.length, addresses: loaded.reduce((sum, row) => sum + row.addresses.length, 0), emails: loaded.reduce((sum, row) => sum + row.emails.length, 0) },
      };
    };
    const teardown = async () => {
      await connection.close();
      await dropIsolated("dictionary_read");
    };
    await setup();
    const read = await performanceDictionaryRead();
    await teardown();
    return { seedMs, readMs: read.ms, readPerSec: read.perSec, counts: read.counts };
  } finally {
    if (connection.isOpen) await connection.close().catch(() => undefined);
  }
};

const performanceTestPropertyMappedRead = async () => {
  const url = await isolate("property_mapped_read");
  await using ds = Datasource.create({ PROVIDER: provider, DATASOURCE_URL: url });
  const connection = await ds.createConnection();
  await connection.open();
  let seedMs = 0;
  try {
    const bulk = contactsBulk(connection);
    const setup = async () => {
      const seeded = seedContacts(rows);
      const started = performance.now();
      await each(seeded, (batch) => bulk.createBulk(batch));
      seedMs = performance.now() - started;
      await countContacts(connection);
    };
    const performancePropertyMappedRead = async () => {
      const repo = ContactsRepository.createReadonly(connection);
      let loaded = await collectAsync(repo.read());
      check(loaded, "propertyMapped");
      const times: number[] = [];
      for (let iter = 0; iter < readIters; iter++) {
        const started = performance.now();
        loaded = await collectAsync(repo.read());
        check(loaded, "propertyMapped");
        times.push(performance.now() - started);
      }
      const ms = median(times);
      return {
        ms,
        perSec: rows / (ms / 1000),
        counts: { contacts: loaded.length, addresses: loaded.reduce((sum, row) => sum + row.addresses.length, 0), emails: loaded.reduce((sum, row) => sum + row.emails.length, 0) },
      };
    };
    const teardown = async () => {
      await connection.close();
      await dropIsolated("property_mapped_read");
    };
    await setup();
    const read = await performancePropertyMappedRead();
    await teardown();
    return { seedMs, readMs: read.ms, readPerSec: read.perSec, counts: read.counts };
  } finally {
    if (connection.isOpen) await connection.close().catch(() => undefined);
  }
};

const startupMs = performance.now() - processStarted;
const [defaultRead, dictionaryRead, propertyMappedRead] = await Promise.all([
  performanceTestDefaultRead(),
  performanceTestDictionaryRead(),
  performanceTestPropertyMappedRead(),
]);
process.stdout.write(
  `${JSON.stringify({
    sample: "mapping_comparison",
    language: "typescript",
    provider,
    rows,
    chunk: Math.min(chunk, rows),
    startupMs,
    tests: { defaultRead, dictionaryRead, propertyMappedRead },
  })}\n`,
);
