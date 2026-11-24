import "../generated/repositories/repositoryRegistry.ts";
import {
  BatchCommandIBulkWriteStrategy,
  BulkCRUDRepository,
  Datasource,
  LoadLevelOrderReadonlyEnrichmentStrategy,
  Operator,
  ReadonlyRepository,
  Where,
} from "@deterministic-code/datasources";
import { AddressesRepository, options as addressOptions } from "../generated/repositories/addressesRepository.ts";
import { AddressTypesRepository } from "../generated/repositories/addressTypesRepository.ts";
import { options as contactOptions } from "../generated/repositories/contactsRepository.ts";
import { ContactPreferencesRepository, options as preferenceOptions } from "../generated/repositories/contactPreferencesRepository.ts";
import { ContactSettingsRepository, options as settingOptions } from "../generated/repositories/contactSettingsRepository.ts";
import { ContactSourcesRepository } from "../generated/repositories/contactSourcesRepository.ts";
import { PhonesRepository, options as phoneOptions } from "../generated/repositories/phonesRepository.ts";
import { PhoneTypesRepository } from "../generated/repositories/phoneTypesRepository.ts";
import { address } from "../generated/types/view/address.ts";
import { contact, type Contact } from "../generated/types/view/contact.ts";
import { phone } from "../generated/types/view/phone.ts";

await using ds = Datasource.default;
const contacts = ds.contacts;
const ada = await contacts.create({
  firstName: "Ada",
  lastName: "Lovelace",
  notes: "poet",
  email: "ada@example.com",
  contactSourceName: "Manual",
  uuid: "00000000-0000-0000-0000-000000000001",
  version: "AA==",
  addresses: [
    { addressTypeName: "Home", line1: "12 Analytical Engine", line2: "Room 12", city: "London", region: "England", postalCode: "SW1A 1AA", country: "GB" },
    { addressTypeName: "Work", line1: "1 Strand", city: "London" },
  ],
  phones: [{ phoneTypeName: "Mobile", number: "555-0100" }],
  settings: { theme: "dark" },
  prefs: { home: { locale: "en-GB", timezone: "Europe/London" } },
});
if (ada.firstName !== "Ada" || ada.addresses.length !== 2 || ada.phones.length !== 1
  || ada.addresses[0].line2 !== "Room 12" || ada.addresses[1].addressTypeName !== "Work")
  throw new Error("create ada");
const loaded = await contacts.readSingle(new Where(new Operator.Equals("id", ada.id)));
if (loaded == null || loaded.addresses[0].latitude != null)
  throw new Error("read ada");
requireAdaChildren(loaded, "Home", "London", "555-0100");
if (loaded.addresses.find((row) => row.line1 === "1 Strand")?.addressTypeName !== "Work")
  throw new Error("read work address");

await contacts.update({
  id: ada.id,
  firstName: "Ada Byron",
  lastName: ada.lastName,
  contactSourceName: "Manual",
  uuid: ada.uuid,
  version: ada.version,
  addresses: [
    { id: ada.addresses[0].id, addressTypeName: "Work", line1: ada.addresses[0].line1, line2: "", city: "Cambridge", region: "", country: "FR" },
    { addressTypeName: "Home", line1: "1 College", city: "Cambridge", country: "GB" },
  ],
  phones: [{ id: ada.phones[0].id, phoneTypeName: "Mobile", number: "555-0111" }],
  settings: { theme: "sepia" },
  prefs: ada.prefs,
});
const after = await contacts.readSingle(new Where(new Operator.Equals("id", ada.id)));
if (after == null || after.firstName !== "Ada Byron"
  || after.addresses.find((row) => row.line1 === "12 Analytical Engine")?.country !== "FR")
  throw new Error("update ada");
requireAdaChildren(after, "Work", "Cambridge", "555-0111");
if (after.addresses.find((row) => row.line1 === "1 College")?.addressTypeName !== "Home")
  throw new Error("update home address");

const omitted = await contacts.create({
  firstName: "Outlier",
  lastName: "Types",
  contactSourceName: "Manual",
  uuid: "00000000-0000-0000-0000-000000000003",
  version: "AA==",
  addresses: [{ addressTypeName: "Home", line1: "1 Null Lane", city: "London" }],
  phones: [],
  settings: {},
  prefs: {},
});
if (omitted.addresses[0].latitude != null || omitted.addresses[0].isVerified != null)
  throw new Error("outlier omit");
await contacts.update({
  id: omitted.id,
  firstName: omitted.firstName,
  lastName: omitted.lastName,
  contactSourceName: "Manual",
  uuid: omitted.uuid,
  version: omitted.version,
  addresses: [{
    id: omitted.addresses[0].id,
    addressTypeName: "Home",
    line1: "1 Null Lane",
    city: "London",
    latitude: 51,
    labelNote: "side",
    floor: 2,
    score: "10.5",
    externalId: "550e8400-e29b-41d4-a716-446655440000",
    isVerified: true,
    verifiedAt: "2020-01-02T03:04:05.000Z",
    stamp: "AQ==",
  }],
  phones: [],
  settings: {},
  prefs: {},
});
const valued = await contacts.readSingle(new Where(new Operator.Equals("id", omitted.id)));
if (valued?.addresses[0].latitude !== 51 || valued.addresses[0].labelNote !== "side"
  || valued.addresses[0].floor !== 2 || valued.addresses[0].score !== "10.5"
  || valued.addresses[0].isVerified !== true)
  throw new Error("outlier value");
await contacts.update({
  id: omitted.id,
  firstName: omitted.firstName,
  lastName: omitted.lastName,
  contactSourceName: "Manual",
  uuid: omitted.uuid,
  version: valued.version,
  addresses: [{
    id: omitted.addresses[0].id,
    addressTypeName: "Home",
    line1: "1 Null Lane",
    city: "London",
    latitude: 0,
    labelNote: "",
    floor: 0,
    score: "0",
    isVerified: false,
  }],
  phones: [],
  settings: {},
  prefs: {},
});
const zeroed = await contacts.readSingle(new Where(new Operator.Equals("id", omitted.id)));
if (zeroed?.addresses[0].latitude !== 0 || zeroed.addresses[0].labelNote !== ""
  || zeroed.addresses[0].floor !== 0 || zeroed.addresses[0].isVerified !== false)
  throw new Error("outlier zero");

const connection = await ds.createConnection();
if (!connection.isOpen) await connection.open();
const addressTypes = new AddressTypesRepository(connection);
const phoneTypes = new PhoneTypesRepository(connection);
const contactSources = new ContactSourcesRepository(connection);
const addressesBulk = new BulkCRUDRepository(connection, addressOptions(connection), new BatchCommandIBulkWriteStrategy());
addressesBulk.addEnriched({ destinationProperty: "addressTypeName", sourceProperty: "addressTypeId", relatedKey: "id", relatedProperty: "name", type: "object" }, addressTypes);
addressesBulk.addEnriched({ destinationProperty: "description", sourceProperty: "addressTypeId", relatedKey: "id", relatedProperty: "description", type: "object" }, addressTypes);
const phonesBulk = new BulkCRUDRepository(connection, phoneOptions(connection), new BatchCommandIBulkWriteStrategy());
phonesBulk.addEnriched({ destinationProperty: "phoneTypeName", sourceProperty: "phoneTypeId", relatedKey: "id", relatedProperty: "name", type: "object" }, phoneTypes);
phonesBulk.addEnriched({ destinationProperty: "description", sourceProperty: "phoneTypeId", relatedKey: "id", relatedProperty: "description", type: "object" }, phoneTypes);
const settingsBulk = new BulkCRUDRepository(connection, settingOptions(connection), new BatchCommandIBulkWriteStrategy());
const prefsBulk = new BulkCRUDRepository(connection, preferenceOptions(connection), new BatchCommandIBulkWriteStrategy());
const contactsBulk = new BulkCRUDRepository(connection, contactOptions(connection), new BatchCommandIBulkWriteStrategy());
contactsBulk.addEnriched({ destinationProperty: "addresses", sourceProperty: "id", relatedKey: "contactId", type: "list" }, addressesBulk);
contactsBulk.addEnriched({ destinationProperty: "phones", sourceProperty: "id", relatedKey: "contactId", type: "list" }, phonesBulk);
contactsBulk.addEnriched({ destinationProperty: "settings", sourceProperty: "id", relatedKey: "contactId", relatedProperty: "value", dictionaryKey: "key", type: "dictionary" }, settingsBulk);
contactsBulk.addEnriched({ destinationProperty: "prefs", sourceProperty: "id", relatedKey: "contactId", dictionaryKey: "key", type: "dictionary" }, prefsBulk);
contactsBulk.addEnriched({ destinationProperty: "contactSourceName", sourceProperty: "contactSourceId", relatedKey: "id", relatedProperty: "name", type: "object" }, contactSources);
contactsBulk.addEnriched({ destinationProperty: "description", sourceProperty: "contactSourceId", relatedKey: "id", relatedProperty: "description", type: "object" }, contactSources);
const bulkItems = Array.from({ length: 10 }, (_, index) => {
  const children = 5 + index % 6;
  return contact({
    firstName: `Bulk${index}`,
    lastName: "Bulk",
    contactSourceName: "Manual",
    addresses: Array.from({ length: children - 1 }, (_, n) => address({
      addressTypeName: n % 2 === 0 ? "Home" : "Work",
      line1: `${index}-${n} Bulk Lane`,
      city: "London",
      externalId: bulkExternalId(index, n),
      verifiedAt: bulkVerifiedAt(),
    })),
    phones: [phone({ phoneTypeName: "Mobile", number: `555-1${String(index).padStart(3, "0")}` })],
  });
});
const createdBulk = await contactsBulk.createBulk(bulkItems);
if (createdBulk.length !== 10 || createdBulk.some((row) => row.id <= 0)
  || new Set(createdBulk.map((row) => row.id)).size !== 10)
  throw new Error("bulk create ids");
createdBulk.forEach((row, index) => requireBulkContact(row, index));
const loadedAll: Contact[] = [];
for await (const row of contacts.read()) loadedAll.push(row);
createdBulk.forEach((created, index) => {
  const loaded = loadedAll.find((row) => row.id === created.id);
  if (loaded == null) throw new Error("bulk read child counts");
  requireBulkContact(loaded, index);
});

const strategy = new LoadLevelOrderReadonlyEnrichmentStrategy<Contact>().withBatchSize(3);
const loadLevel = new ReadonlyRepository<Contact>(connection, contactOptions(connection));
const loadLevelAddresses = new AddressesRepository(connection);
const loadLevelPhones = new PhonesRepository(connection);
const loadLevelSettings = new ContactSettingsRepository(connection);
const loadLevelPrefs = new ContactPreferencesRepository(connection);
const loadLevelSources = new ContactSourcesRepository(connection);
loadLevel.addEnriched({ destinationProperty: "addresses", sourceProperty: "id", relatedKey: "contactId", type: "list" }, loadLevelAddresses, strategy);
loadLevel.addEnriched({ destinationProperty: "phones", sourceProperty: "id", relatedKey: "contactId", type: "list" }, loadLevelPhones, strategy);
loadLevel.addEnriched({ destinationProperty: "settings", sourceProperty: "id", relatedKey: "contactId", relatedProperty: "value", dictionaryKey: "key", type: "dictionary" }, loadLevelSettings, strategy);
loadLevel.addEnriched({ destinationProperty: "prefs", sourceProperty: "id", relatedKey: "contactId", dictionaryKey: "key", type: "dictionary" }, loadLevelPrefs, strategy);
loadLevel.addEnriched({ destinationProperty: "contactSourceName", sourceProperty: "contactSourceId", relatedKey: "id", relatedProperty: "name", type: "object" }, loadLevelSources, strategy);
loadLevel.addEnriched({ destinationProperty: "description", sourceProperty: "contactSourceId", relatedKey: "id", relatedProperty: "description", type: "object" }, loadLevelSources, strategy);
const loadLevelAda = await loadLevel.readSingle(new Where(new Operator.Equals("id", after.id)));
if (loadLevelAda == null || loadLevelAda.firstName !== "Ada Byron"
  || loadLevelAda.settings.theme !== "sepia"
  || loadLevelAda.prefs.home?.locale !== "en-GB")
  throw new Error("load-level-order ada");
requireAdaChildren(loadLevelAda, "Work", "Cambridge", "555-0111");
const loadLevelAll: Contact[] = [];
for await (const row of loadLevel.read()) loadLevelAll.push(row);
createdBulk.forEach((created, index) => {
  const loaded = loadLevelAll.find((row) => row.id === created.id);
  if (loaded == null) throw new Error("load-level-order bulk");
  requireBulkContact(loaded, index);
});

await contacts.delete(after);
if ((await contacts.readSingle(new Where(new Operator.Equals("id", ada.id)))) != null)
  throw new Error("delete ada");

function bulkExternalId(index: number, n: number): string {
  return `00000000-0000-0000-0001-${String(index).padStart(4, "0")}${String(n).padStart(8, "0")}`;
}

function bulkVerifiedAt(): Date {
  return new Date("2020-01-02T03:04:05.000Z");
}

function assignedUuid(value: string | null | undefined): boolean {
  return value != null && value !== "" && value !== "00000000-0000-0000-0000-000000000000";
}

function assignedDate(value: Date | null | undefined): boolean {
  return value instanceof Date && !Number.isNaN(value.getTime()) && value.getUTCFullYear() >= 2020;
}

function requireBulkContact(row: Contact, index: number) {
  if (row.firstName !== `Bulk${index}` || row.lastName !== "Bulk" || row.contactSourceName !== "Manual")
    throw new Error("bulk contact fields");
  if (row.id <= 0 || !assignedUuid(row.uuid)) throw new Error("bulk contact uuid");
  if (!assignedDate(row.created) || !assignedDate(row.updated)) throw new Error("bulk contact datetime");
  const children = 5 + index % 6;
  if (row.addresses.length + row.phones.length !== children) throw new Error("bulk child counts");
  for (let n = 0; n < children - 1; n++) {
    const address = row.addresses.find((item) => item.line1 === `${index}-${n} Bulk Lane`);
    if (address == null || address.addressTypeName !== (n % 2 === 0 ? "Home" : "Work") || address.city !== "London")
      throw new Error("bulk address fields");
    if (address.contactId !== row.id) throw new Error("bulk address contact id");
    if (address.id <= 0 || !assignedUuid(address.uuid) || address.externalId !== bulkExternalId(index, n))
      throw new Error("bulk address uuid");
    if (!assignedDate(address.created) || !assignedDate(address.updated)
      || address.verifiedAt?.getTime() !== bulkVerifiedAt().getTime())
      throw new Error("bulk address datetime");
  }
  const phone = row.phones[0];
  if (phone.number !== `555-1${String(index).padStart(3, "0")}` || phone.phoneTypeName !== "Mobile")
    throw new Error("bulk phone fields");
  if (phone.contactId !== row.id) throw new Error("bulk phone contact id");
  if (phone.id <= 0 || !assignedUuid(phone.uuid)) throw new Error("bulk phone uuid");
  if (!assignedDate(phone.created) || !assignedDate(phone.updated)) throw new Error("bulk phone datetime");
}

function requireAdaChildren(row: Contact, engineType: string, engineCity: string, number: string) {
  if (row.addresses.length !== 2 || row.phones.length !== 1) throw new Error("child counts");
  const engine = row.addresses.find((address) => address.line1 === "12 Analytical Engine");
  if (engine?.addressTypeName !== engineType || engine.city !== engineCity) throw new Error("engine address");
  const addressFk: Contact["id"] = engine.contactId;
  if (addressFk !== row.id) throw new Error("address contactId");
  if (row.phones[0].number !== number || row.phones[0].phoneTypeName !== "Mobile") throw new Error("phone");
  const phoneFk: Contact["id"] = row.phones[0].contactId;
  if (phoneFk !== row.id) throw new Error("phone contactId");
}
