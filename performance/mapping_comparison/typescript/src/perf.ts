import {
  BatchCommandIBulkWriteStrategy,
  BulkCRUDRepository,
  ChangeCasingMapper,
  Datasource,
  DatasourceFactory,
  DatasourceUrlParser,
  DictionaryMapper,
  GraphReadonlyEnrichmentStrategy,
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
import { AddressesRepository } from "../generated/repositories/addressesRepository.ts";
import { ContactAttributesRepository } from "../generated/repositories/contactAttributesRepository.ts";
import { ContactLabelsRepository } from "../generated/repositories/contactLabelsRepository.ts";
import { ContactsRepository } from "../generated/repositories/contactsRepository.ts";
import { EmailsRepository } from "../generated/repositories/emailsRepository.ts";
import { LabelsRepository } from "../generated/repositories/labelsRepository.ts";
import { PhonesRepository } from "../generated/repositories/phonesRepository.ts";
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
const startupMs = performance.now() - processStarted;

const of = <T extends object>(create: string, update: string, remove: string): IBulkWriteStrategy<T> => {
  if (provider === "sqlite") return new BatchCommandIBulkWriteStrategy<T>();
  return new StoredProcedureBulkWriteStrategy<T>(create, update, remove);
};

const each = async <T>(items: T[], write: (batch: T[]) => Promise<unknown>) => {
  for (let at = 0; at < items.length; at += chunk) await write(items.slice(at, at + chunk));
};

const average = (values: number[]) => values.reduce((sum, value) => sum + value, 0) / values.length;

const cpuMs = (from: NodeJS.CpuUsage) => {
  const used = process.cpuUsage(from);
  return (used.user + used.system) / 1000;
};

const seedContacts = (total: number) => {
  const contacts: Contact[] = [];
  for (let i = 1; i <= total; i++) {
    contacts.push(
      contact({
        Name: `Contact ${i}`,
        GivenName: `Given${i}`,
        FamilyName: `Family${i}`,
        GenderTypeId: 1 + (i % 3),
        IsStarred: i % 2 === 0,
        IsFavorite: i % 3 === 0,
        IsDeleted: i % 5 === 0,
        Addresses: [
          address({ Line1: `${i} Line 1`, City: `City${i % 50}`, CountryCode: "US", IsPrimary: true, Latitude: null, Longitude: null }),
          address({ Line1: `${i} Line 2`, City: `City${(i + 1) % 50}`, CountryCode: "US", IsPrimary: false, Latitude: null, Longitude: null }),
        ],
        Emails: [
          email({ Address: `c${i}.e1@example.com`, DisplayName: "Email 1", EmailTypeId: 1, IsPrimary: true }),
          email({ Address: `c${i}.e2@example.com`, DisplayName: "Email 2", EmailTypeId: 2, IsPrimary: false }),
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
    if (row.Addresses.length !== 2 || row.Emails.length !== 2) throw new Error(`${label} related ${row.GivenName}`);
    if (row.Phones.length !== 0 || Object.keys(row.Attributes).length !== 0 || row.Labels.length !== 0)
      throw new Error(`${label} empty fills ${row.GivenName}`);
    if (!uuid.test(row.Uuid) || row.Addresses.some((item) => !uuid.test(item.Uuid)) || row.Emails.some((item) => !uuid.test(item.Uuid)))
      throw new Error(`${label} uuid ${row.GivenName}`);
    if (!mappedDate(row.Created) || !mappedDate(row.Updated)
      || row.Addresses.some((item) => !mappedDate(item.Created) || !mappedDate(item.Updated))
      || row.Emails.some((item) => !mappedDate(item.Created) || !mappedDate(item.Updated)))
      throw new Error(`${label} datetime ${row.GivenName}`);
    if (!row.GivenName.startsWith("Given")) throw new Error(`${label} givenName ${row.GivenName}`);
    const n = Number(row.GivenName.slice("Given".length));
    if (!Number.isInteger(n) || row.IsStarred !== (n % 2 === 0) || row.IsFavorite !== (n % 3 === 0) || row.IsDeleted !== (n % 5 === 0))
      throw new Error(`${label} boolean ${row.GivenName}`);
    if (!row.Addresses[0]!.IsPrimary || row.Addresses[1]!.IsPrimary || !row.Emails[0]!.IsPrimary || row.Emails[1]!.IsPrimary)
      throw new Error(`${label} isPrimary ${row.GivenName}`);
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
  const addressesWrite = new BulkCRUDRepository(
    connection,
    { type: Address, entity: "Address", entityMapper: new DictionaryMapper([["Address", "addresses"]]), propertyMappings: new DictionaryMapper([["Id", "id"], ["Uuid", "uuid"], ["Created", "created"], ["Updated", "updated"], ["Version", "version"], ["ContactId", "contact_id"], ["DisplayName", "display_name"], ["Line1", "line_1"], ["Line2", "line_2"], ["Line3", "line_3"], ["Neighborhood", "neighborhood"], ["District", "district"], ["City", "city"], ["StateProvince", "state_province"], ["PostalCode", "postal_code"], ["CountryCode", "country_code"], ["FormattedAddress", "formatted_address"], ["Latitude", "latitude"], ["Longitude", "longitude"], ["IsPrimary", "is_primary"]]), ids: ["Id"], createProperties: ["Uuid", "Created", "Updated", "Version", "ContactId", "DisplayName", "Line1", "Line2", "Line3", "Neighborhood", "District", "City", "StateProvince", "PostalCode", "CountryCode", "FormattedAddress", "Latitude", "Longitude", "IsPrimary"], updateProperties: ["Uuid", "Created", "Updated", "Version", "ContactId", "DisplayName", "Line1", "Line2", "Line3", "Neighborhood", "District", "City", "StateProvince", "PostalCode", "CountryCode", "FormattedAddress", "Latitude", "Longitude", "IsPrimary"], readProperties: ["Id", "Uuid", "Created", "Updated", "Version", "ContactId", "DisplayName", "Line1", "Line2", "Line3", "Neighborhood", "District", "City", "StateProvince", "PostalCode", "CountryCode", "FormattedAddress", "Latitude", "Longitude", "IsPrimary"], schema: new Schema({ Id: { type: "integer", nullable: false, hasDefault: false, isId: true }, Uuid: { type: "uuid", nullable: false, hasDefault: true }, Created: { type: "datetime", nullable: false, hasDefault: true }, Updated: { type: "datetime", nullable: false, hasDefault: true }, Version: { type: "binary", nullable: false, hasDefault: true }, ContactId: { type: "integer", nullable: false, hasDefault: false }, DisplayName: { type: "string", nullable: false, hasDefault: true }, Line1: { type: "string", nullable: false, hasDefault: false }, Line2: { type: "string", nullable: false, hasDefault: true }, Line3: { type: "string", nullable: false, hasDefault: true }, Neighborhood: { type: "string", nullable: false, hasDefault: true }, District: { type: "string", nullable: false, hasDefault: true }, City: { type: "string", nullable: false, hasDefault: false }, StateProvince: { type: "string", nullable: false, hasDefault: true }, PostalCode: { type: "string", nullable: false, hasDefault: true }, CountryCode: { type: "string", nullable: false, hasDefault: false }, FormattedAddress: { type: "string", nullable: false, hasDefault: true }, Latitude: { type: "decimal", nullable: true, hasDefault: false }, Longitude: { type: "decimal", nullable: true, hasDefault: false }, IsPrimary: { type: "boolean", nullable: false, hasDefault: true } }) },
    of<Address>("bulk_create_addresses", "bulk_update_addresses", "bulk_delete_addresses"),
  );
  const emailsWrite = new BulkCRUDRepository(
    connection,
    { type: Email, entity: "Email", entityMapper: new DictionaryMapper([["Email", "emails"]]), propertyMappings: new DictionaryMapper([["Id", "id"], ["Uuid", "uuid"], ["Created", "created"], ["Updated", "updated"], ["Version", "version"], ["EmailTypeId", "email_type_id"], ["ContactId", "contact_id"], ["Address", "address"], ["DisplayName", "display_name"], ["IsPrimary", "is_primary"]]), ids: ["Id"], createProperties: ["Uuid", "Created", "Updated", "Version", "EmailTypeId", "ContactId", "Address", "DisplayName", "IsPrimary"], updateProperties: ["Uuid", "Created", "Updated", "Version", "EmailTypeId", "ContactId", "Address", "DisplayName", "IsPrimary"], readProperties: ["Id", "Uuid", "Created", "Updated", "Version", "EmailTypeId", "ContactId", "Address", "DisplayName", "IsPrimary"], schema: new Schema({ Id: { type: "integer", nullable: false, hasDefault: false, isId: true }, Uuid: { type: "uuid", nullable: false, hasDefault: true }, Created: { type: "datetime", nullable: false, hasDefault: true }, Updated: { type: "datetime", nullable: false, hasDefault: true }, Version: { type: "binary", nullable: false, hasDefault: true }, EmailTypeId: { type: "integer", nullable: false, hasDefault: false }, ContactId: { type: "integer", nullable: false, hasDefault: false }, Address: { type: "string", nullable: false, hasDefault: false }, DisplayName: { type: "string", nullable: false, hasDefault: true }, IsPrimary: { type: "boolean", nullable: false, hasDefault: false }, EmailTypeName: { type: "string", nullable: false, hasDefault: false }, Description: { type: "string", nullable: false, hasDefault: true } }) },
    of<Email>("bulk_create_emails", "bulk_update_emails", "bulk_delete_emails"),
  );
  const bulk = new BulkCRUDRepository(
    connection,
    { type: Contact, entity: "Contact", entityMapper: new DictionaryMapper([["Contact", "contacts"]]), propertyMappings: new DictionaryMapper([["Id", "id"], ["Uuid", "uuid"], ["Created", "created"], ["Updated", "updated"], ["Version", "version"], ["Name", "name"], ["GivenName", "given_name"], ["AdditionalName", "additional_name"], ["Pronunciation", "pronunciation"], ["FamilyName", "family_name"], ["NickName", "nick_name"], ["JobTitle", "job_title"], ["GenderTypeId", "gender_type_id"], ["Organization", "organization"], ["IsStarred", "is_starred"], ["IsFavorite", "is_favorite"], ["IsDeleted", "is_deleted"]]), ids: ["Id"], createProperties: ["Uuid", "Created", "Updated", "Version", "Name", "GivenName", "AdditionalName", "Pronunciation", "FamilyName", "NickName", "JobTitle", "GenderTypeId", "Organization", "IsStarred", "IsFavorite", "IsDeleted"], updateProperties: ["Uuid", "Created", "Updated", "Version", "Name", "GivenName", "AdditionalName", "Pronunciation", "FamilyName", "NickName", "JobTitle", "GenderTypeId", "Organization", "IsStarred", "IsFavorite", "IsDeleted"], readProperties: ["Id", "Uuid", "Created", "Updated", "Version", "Name", "GivenName", "AdditionalName", "Pronunciation", "FamilyName", "NickName", "JobTitle", "GenderTypeId", "Organization", "IsStarred", "IsFavorite", "IsDeleted"], schema: new Schema({ Id: { type: "integer", nullable: false, hasDefault: false, isId: true }, Uuid: { type: "uuid", nullable: false, hasDefault: true }, Created: { type: "datetime", nullable: false, hasDefault: true }, Updated: { type: "datetime", nullable: false, hasDefault: true }, Version: { type: "binary", nullable: false, hasDefault: true }, Name: { type: "string", nullable: false, hasDefault: false }, GivenName: { type: "string", nullable: false, hasDefault: false }, AdditionalName: { type: "string", nullable: false, hasDefault: true }, Pronunciation: { type: "string", nullable: false, hasDefault: true }, FamilyName: { type: "string", nullable: false, hasDefault: true }, NickName: { type: "string", nullable: false, hasDefault: true }, JobTitle: { type: "string", nullable: false, hasDefault: true }, GenderTypeId: { type: "integer", nullable: false, hasDefault: false }, Organization: { type: "string", nullable: false, hasDefault: true }, IsStarred: { type: "boolean", nullable: false, hasDefault: true }, IsFavorite: { type: "boolean", nullable: false, hasDefault: true }, IsDeleted: { type: "boolean", nullable: false, hasDefault: true } }) },
    of<Contact>("bulk_create_contacts", "bulk_update_contacts", "bulk_delete_contacts"),
  );
  bulk.addEnriched({ destinationProperty: "Addresses", sourceProperty: "Id", relatedKey: "ContactId", type: "list" }, addressesWrite);
  bulk.addEnriched({ destinationProperty: "Emails", sourceProperty: "Id", relatedKey: "ContactId", type: "list" }, emailsWrite);
  return bulk;
};

const performanceTestDefaultRead = async () => {
  const url = await isolate("default_read");
  await using ds = Datasource.create({ PROVIDER: provider, DATASOURCE_URL: url });
  const connection = await ds.createConnection();
  await connection.open();
  let seedMs = 0;
  let seedCpuMs = 0;
  try {
    const bulk = contactsBulk(connection);
    const setup = async () => {
      const seeded = seedContacts(rows);
      const cpu = process.cpuUsage();
      const started = performance.now();
      await each(seeded, (batch) => bulk.createBulk(batch));
      seedMs = performance.now() - started;
      seedCpuMs = cpuMs(cpu);
      await countContacts(connection);
    };
    const performanceDefaultRead = async () => {
      const repo = ds.createRepository<Contact>("Contacts");
      let loaded = await collectAsync(repo.read());
      check(loaded, "default");
      const times: number[] = [];
      const cpu = process.cpuUsage();
      for (let iter = 0; iter < readIters; iter++) {
        const started = performance.now();
        loaded = await collectAsync(repo.read());
        times.push(performance.now() - started);
      }
      const cpuUsed = cpuMs(cpu);
      check(loaded, "default");
      const ms = average(times);
      return {
        ms,
        cpuMs: cpuUsed,
        perSec: rows / (ms / 1000),
        counts: { contacts: loaded.length, addresses: loaded.reduce((sum, row) => sum + row.Addresses.length, 0), emails: loaded.reduce((sum, row) => sum + row.Emails.length, 0) },
      };
    };
    const teardown = async () => {
      await connection.close();
      await dropIsolated("default_read");
    };
    await setup();
    const read = await performanceDefaultRead();
    await teardown();
    return { seedMs, seedCpuMs, readMs: read.ms, readCpuMs: read.cpuMs, readPerSec: read.perSec, counts: read.counts };
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
  let seedCpuMs = 0;
  try {
    const snake = new ChangeCasingMapper("pascalCase", "snakeCase");
    const props = (names: string[]) => new DictionaryMapper(names.map((name) => [name, snake.map(name)]));
    const contactColumns = [
      "Id", "Uuid", "Created", "Updated", "Version", "Name", "GivenName", "AdditionalName", "Pronunciation",
      "FamilyName", "NickName", "JobTitle", "Organization", "IsStarred", "IsFavorite", "IsDeleted", "GenderTypeId",
    ];
    const bulk = contactsBulk(connection);
    const contacts = new ReadonlyRepository(connection, {
      type: Contact,
      entity: "Contact",
      entityMapper: new DictionaryMapper([["Contact", "contacts"]]),
      propertyMappings: props([...contactColumns]),
      readProperties: [...contactColumns],
      schema: new Schema({
        Id: { type: "integer", nullable: false, hasDefault: false, isId: true },
        Uuid: { type: "uuid", nullable: false, hasDefault: true },
        Created: { type: "datetime", nullable: false, hasDefault: true },
        Updated: { type: "datetime", nullable: false, hasDefault: true },
        Version: { type: "binary", nullable: false, hasDefault: true },
        Name: { type: "string", nullable: false, hasDefault: false },
        GivenName: { type: "string", nullable: false, hasDefault: false },
        AdditionalName: { type: "string", nullable: false, hasDefault: true },
        Pronunciation: { type: "string", nullable: false, hasDefault: true },
        FamilyName: { type: "string", nullable: false, hasDefault: true },
        NickName: { type: "string", nullable: false, hasDefault: true },
        JobTitle: { type: "string", nullable: false, hasDefault: true },
        Organization: { type: "string", nullable: false, hasDefault: true },
        IsStarred: { type: "boolean", nullable: false, hasDefault: false },
        IsFavorite: { type: "boolean", nullable: false, hasDefault: false },
        IsDeleted: { type: "boolean", nullable: false, hasDefault: false },
        GenderTypeId: { type: "integer", nullable: false, hasDefault: false },
      }),
    });
    const Address = new AddressesRepository(connection);
    const Email = new EmailsRepository(connection);
    const Phone = new PhonesRepository(connection);
    const ContactAttribute = new ContactAttributesRepository(connection);
    const Label = new LabelsRepository(connection);
    const ContactLabel = new ContactLabelsRepository(connection);
    contacts.addEnriched({ destinationProperty: "Addresses", sourceProperty: "Id", relatedKey: "ContactId", type: "list" }, Address);
    contacts.addEnriched({ destinationProperty: "Emails", sourceProperty: "Id", relatedKey: "ContactId", type: "list" }, Email);
    contacts.addEnriched({ destinationProperty: "Phones", sourceProperty: "Id", relatedKey: "ContactId", type: "list" }, Phone);
    contacts.addEnriched({ destinationProperty: "Attributes", sourceProperty: "Id", relatedKey: "ContactId", relatedProperty: "Value", dictionaryKey: "Name", type: "dictionary" }, ContactAttribute);
    contacts.addEnriched({ destinationProperty: "Labels", sourceProperty: "Id", relatedKey: "Id", relatedProperty: "Name", association: { parent: "ContactId", related: "LabelId" }, type: "list" }, Label, new GraphReadonlyEnrichmentStrategy(ContactLabel));
    const setup = async () => {
      const seeded = seedContacts(rows);
      const cpu = process.cpuUsage();
      const started = performance.now();
      await each(seeded, (batch) => bulk.createBulk(batch));
      seedMs = performance.now() - started;
      seedCpuMs = cpuMs(cpu);
      await countContacts(connection);
    };
    const performanceDictionaryRead = async () => {
      let loaded = await collectAsync(contacts.read());
      check(loaded, "dictionary");
      const times: number[] = [];
      const cpu = process.cpuUsage();
      for (let iter = 0; iter < readIters; iter++) {
        const started = performance.now();
        loaded = await collectAsync(contacts.read());
        times.push(performance.now() - started);
      }
      const cpuUsed = cpuMs(cpu);
      check(loaded, "dictionary");
      const ms = average(times);
      return {
        ms,
        cpuMs: cpuUsed,
        perSec: rows / (ms / 1000),
        counts: { contacts: loaded.length, addresses: loaded.reduce((sum, row) => sum + row.Addresses.length, 0), emails: loaded.reduce((sum, row) => sum + row.Emails.length, 0) },
      };
    };
    const teardown = async () => {
      await connection.close();
      await dropIsolated("dictionary_read");
    };
    await setup();
    const read = await performanceDictionaryRead();
    await teardown();
    return { seedMs, seedCpuMs, readMs: read.ms, readCpuMs: read.cpuMs, readPerSec: read.perSec, counts: read.counts };
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
  let seedCpuMs = 0;
  try {
    const bulk = contactsBulk(connection);
    const setup = async () => {
      const seeded = seedContacts(rows);
      const cpu = process.cpuUsage();
      const started = performance.now();
      await each(seeded, (batch) => bulk.createBulk(batch));
      seedMs = performance.now() - started;
      seedCpuMs = cpuMs(cpu);
      await countContacts(connection);
    };
    const performancePropertyMappedRead = async () => {
      const repo = ContactsRepository.createReadonly(connection);
      let loaded = await collectAsync(repo.read());
      check(loaded, "propertyMapped");
      const times: number[] = [];
      const cpu = process.cpuUsage();
      for (let iter = 0; iter < readIters; iter++) {
        const started = performance.now();
        loaded = await collectAsync(repo.read());
        times.push(performance.now() - started);
      }
      const cpuUsed = cpuMs(cpu);
      check(loaded, "propertyMapped");
      const ms = average(times);
      return {
        ms,
        cpuMs: cpuUsed,
        perSec: rows / (ms / 1000),
        counts: { contacts: loaded.length, addresses: loaded.reduce((sum, row) => sum + row.Addresses.length, 0), emails: loaded.reduce((sum, row) => sum + row.Emails.length, 0) },
      };
    };
    const teardown = async () => {
      await connection.close();
      await dropIsolated("property_mapped_read");
    };
    await setup();
    const read = await performancePropertyMappedRead();
    await teardown();
    return { seedMs, seedCpuMs, readMs: read.ms, readCpuMs: read.cpuMs, readPerSec: read.perSec, counts: read.counts };
  } finally {
    if (connection.isOpen) await connection.close().catch(() => undefined);
  }
};

const defaultRead = await performanceTestDefaultRead();
const dictionaryRead = await performanceTestDictionaryRead();
const propertyMappedRead = await performanceTestPropertyMappedRead();
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
