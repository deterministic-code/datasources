import {
  BatchCommandIBulkWriteStrategy,
  BulkCRUDRepository,
  ChangeCasingMapper,
  Datasource,
  DictionaryMapper,
  Native,
  ReadonlyRepository,
  Schema,
  StoredProcedureBulkWriteStrategy,
  collectAsync,
  type IBulkWriteStrategy,
  type IConnection,
} from "@deterministic-code/datasources";
import "../generated/repositories/repositoryRegistry.ts";
import { Address, address } from "../generated/types/view/address.ts";
import { Contact, contact } from "../generated/types/view/contact.ts";
import { Email, email } from "../generated/types/view/email.ts";

const provider = process.env.PROVIDER ?? "sqlite";
const rows = Number(process.env.PERF_ROWS ?? 25000);
const chunk = Number(process.env.PERF_CHUNK ?? 500);
const readIters = Number(process.env.PERF_READ_ITERS ?? 3);
const snake = new ChangeCasingMapper("camelCase", "snakeCase");
const props = (names: string[]) => new DictionaryMapper(names.map((name) => [name, snake.map(name)]));

const contactColumns = [
  "id",
  "uuid",
  "created",
  "updated",
  "version",
  "name",
  "givenName",
  "additionalName",
  "pronunciation",
  "familyName",
  "nickName",
  "jobTitle",
  "organization",
  "isStarred",
  "isFavorite",
  "isDeleted",
  "genderTypeId",
] as const;

const addressColumns = [
  "id",
  "uuid",
  "created",
  "updated",
  "version",
  "contactId",
  "displayName",
  "line1",
  "line2",
  "line3",
  "neighborhood",
  "district",
  "city",
  "stateProvince",
  "postalCode",
  "countryCode",
  "formattedAddress",
  "latitude",
  "longitude",
  "isPrimary",
] as const;

const emailColumns = [
  "id",
  "uuid",
  "created",
  "updated",
  "version",
  "emailTypeId",
  "contactId",
  "address",
  "displayName",
  "isPrimary",
] as const;

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

const of = <T extends object>(create: string, update: string, remove: string): IBulkWriteStrategy<T> => {
  if (provider === "sqlite") return new BatchCommandIBulkWriteStrategy<T>();
  return new StoredProcedureBulkWriteStrategy<T>(create, update, remove);
};

const each = async <T>(items: T[], write: (batch: T[]) => Promise<unknown>) => {
  for (let at = 0; at < items.length; at += chunk) await write(items.slice(at, at + chunk));
};

const next = (state: { n: number }) => {
  state.n = (Math.imul(1664525, state.n) + 1013904223) >>> 0;
  return state.n / 0x100000000;
};

const relatedCount = (state: { n: number }, max = 10) => 2 + Math.floor(next(state) * (max - 1));

const count = async (connection: IConnection, table: string) => {
  const reader = await connection.createCommand().executeReaderNative(new Native(`SELECT COUNT(*) AS n FROM ${table}`));
  await reader.next();
  const n = reader.current?.n;
  return typeof n === "number" ? n : Number(n);
};

const median = (values: number[]) => {
  const sorted = [...values].sort((left, right) => left - right);
  const mid = Math.floor(sorted.length / 2);
  return sorted.length % 2 === 1 ? sorted[mid]! : (sorted[mid - 1]! + sorted[mid]!) / 2;
};

const time = async (work: () => Promise<void>) => {
  const started = performance.now();
  await work();
  return performance.now() - started;
};

const seedContacts = (total: number) => {
  const state = { n: 1 };
  const contacts: Contact[] = [];
  for (let i = 1; i <= total; i++) {
    const addresses = relatedCount(state);
    const emails = relatedCount(state, 6);
    contacts.push(
      contact({
        id: i,
        name: `Contact ${i}`,
        givenName: `Given${i}`,
        familyName: `Family${i}`,
        genderTypeId: 1 + (i % 3),
        isStarred: false,
        isFavorite: false,
        isDeleted: false,
        addresses: Array.from({ length: addresses }, (_, j) =>
          address({
            line1: `${i} Line ${j + 1}`,
            city: `City${(i + j) % 50}`,
            countryCode: "US",
            isPrimary: j === 0,
            latitude: null,
            longitude: null,
          }),
        ),
        emails: Array.from({ length: emails }, (_, j) =>
          email({
            address: `c${i}.e${j + 1}@example.com`,
            displayName: `Email ${j + 1}`,
            emailTypeId: j + 1,
            isPrimary: j === 0,
          }),
        ),
      }),
    );
  }
  return contacts;
};

await using ds = Datasource.default;
const connection = await ds.createConnection();
await connection.open();
try {
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

  const addressesBulk = new BulkCRUDRepository(
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
  const emailsBulk = new BulkCRUDRepository(
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
  const contactsBulk = new BulkCRUDRepository(
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
  contactsBulk.addEnriched(
    { destinationProperty: "addresses", sourceProperty: "id", relatedKey: "contactId", type: "list" },
    addressesBulk,
  );
  contactsBulk.addEnriched(
    { destinationProperty: "emails", sourceProperty: "id", relatedKey: "contactId", type: "list" },
    emailsBulk,
  );

  const seeded = seedContacts(rows);
  const createStarted = performance.now();
  await each(seeded, (batch) => contactsBulk.createBulk(batch));
  const createMs = performance.now() - createStarted;
  const counts = {
    contacts: await count(connection, "contacts"),
    addresses: await count(connection, "addresses"),
    emails: await count(connection, "emails"),
  };
  const expectedAddresses = seeded.reduce((sum, row) => sum + row.addresses.length, 0);
  const expectedEmails = seeded.reduce((sum, row) => sum + row.emails.length, 0);
  if (counts.contacts !== rows || counts.addresses !== expectedAddresses || counts.emails !== expectedEmails)
    throw new Error(`create count ${JSON.stringify(counts)} expected addresses=${expectedAddresses} emails=${expectedEmails}`);

  const defaultRepo = ds.contacts;
  const dictionaryAddresses = new ReadonlyRepository(connection, {
    type: Address,
    entity: "Address",
    entityMapper: new DictionaryMapper([["Address", "addresses"]]),
    propertyMappings: props([...addressColumns]),
    readProperties: [...addressColumns],
    schema: addressSchema,
  });
  const dictionaryEmails = new ReadonlyRepository(connection, {
    type: Email,
    entity: "Email",
    entityMapper: new DictionaryMapper([["Email", "emails"]]),
    propertyMappings: props([...emailColumns]),
    readProperties: [...emailColumns],
    schema: emailSchema,
  });
  const dictionaryContacts = new ReadonlyRepository(connection, {
    type: Contact,
    entity: "Contact",
    entityMapper: new DictionaryMapper([["Contact", "contacts"]]),
    propertyMappings: props([...contactColumns]),
    readProperties: [...contactColumns],
    schema: contactSchema,
  });
  dictionaryContacts.addEnriched(
    { destinationProperty: "addresses", sourceProperty: "id", relatedKey: "contactId", type: "list" },
    dictionaryAddresses,
  );
  dictionaryContacts.addEnriched(
    { destinationProperty: "emails", sourceProperty: "id", relatedKey: "contactId", type: "list" },
    dictionaryEmails,
  );

  const check = (loaded: Contact[], label: string) => {
    if (loaded.length !== rows) throw new Error(`${label} count ${loaded.length}`);
    if (loaded.some((row) => row.addresses.length < 2 || row.addresses.length > 10 || row.emails.length < 2 || row.emails.length > 6))
      throw new Error(`${label} related`);
  };

  const profile = async (label: "default" | "dictionary", read: () => Promise<Contact[]>) => {
    await read();
    const times: number[] = [];
    for (let iter = 0; iter < readIters; iter++) {
      times.push(await time(async () => check(await read(), label)));
    }
    const ms = median(times);
    return { ms, perSec: rows / (ms / 1000) };
  };

  const reads = {
    default: await profile("default", () => collectAsync(defaultRepo.read())),
    dictionary: await profile("dictionary", () => collectAsync(dictionaryContacts.read())),
  };

  process.stdout.write(
    `${JSON.stringify({
      sample: "mapping_comparison",
      language: "typescript",
      provider,
      rows,
      chunk: Math.min(chunk, rows),
      createMs,
      createPerSec: createMs === 0 ? 0 : rows / (createMs / 1000),
      counts,
      reads,
    })}\n`,
  );
} finally {
  await connection.close().catch(() => undefined);
}
