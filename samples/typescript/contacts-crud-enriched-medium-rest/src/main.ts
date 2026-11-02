import "../generated/typescript/repositories/repositoryRegistry.ts";
import { Datasource, Operator, Where } from "@deterministic-code/datasources";

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
if (loaded?.addresses.length !== 2 || loaded.addresses[0].latitude != null)
  throw new Error("read ada");

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
if (after?.firstName !== "Ada Byron" || after.addresses.length !== 2
  || after.addresses.find((row) => row.line1 === "12 Analytical Engine")?.addressTypeName !== "Work"
  || after.addresses.find((row) => row.line1 === "12 Analytical Engine")?.country !== "FR")
  throw new Error("update ada");

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

await contacts.delete(after);
if ((await contacts.readSingle(new Where(new Operator.Equals("id", ada.id)))) != null)
  throw new Error("delete ada");
