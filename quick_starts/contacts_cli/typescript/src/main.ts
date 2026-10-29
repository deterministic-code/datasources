import "../generated/typescript/repositories/repositoryRegistry.ts";
import { Datasource, Operator, Where, type ICRUDRepository } from "@deterministic-code/datasources";
import type { Address } from "../generated/typescript/types/generated/views/address.ts";
import type { Contact } from "../generated/typescript/types/generated/views/contact.ts";
import type { Email } from "../generated/typescript/types/generated/views/email.ts";
import type { Phone } from "../generated/typescript/types/generated/views/phone.ts";

await using ds = Datasource.default;
const contactsRepo = ds.createRepository("contacts") as ICRUDRepository<Contact>;
const ada = await contactsRepo.create({
  givenName: "Ada",
  genderTypeName: "Female",
  emails: [{ emailTypeName: "Personal", address: "ada@example.com", displayName: "Ada", isPrimary: true } as Email],
  phones: [{ phoneTypeName: "Mobile", number: "555-0100", displayName: "Ada mobile", extention: "", isPrimary: true } as Phone],
  addresses: [{ addressTypeName: "Home", line1: "12 Analytical Engine", city: "London", countryCode: "GB" } as Address],
} as Contact);
console.log(`created ${ada.id} ${ada.givenName} ${ada.emails[0].address} ${ada.phones[0].number} ${ada.addresses[0].line1}`);

const alan = await contactsRepo.create({
  givenName: "Alan",
  genderTypeName: "Male",
  emails: [{ emailTypeName: "Work", address: "alan@work.example", displayName: "Alan", isPrimary: true } as Email],
  phones: [{ phoneTypeName: "Work", number: "555-0200", displayName: "Alan office", extention: "", isPrimary: true } as Phone],
  addresses: [{ addressTypeName: "Work", line1: "1 Bletchley Park", city: "Bletchley", countryCode: "GB" } as Address],
} as Contact);
console.log(`created ${alan.id} ${alan.givenName} ${alan.emails[0].address} ${alan.phones[0].number} ${alan.addresses[0].line1}`);

const adaRead = await contactsRepo.readSingle(new Where(new Operator.Equals("id", ada.id)));
if (adaRead == null) throw new Error("read");
console.log(`read ${adaRead.id} ${adaRead.givenName} emails=${adaRead.emails.length} phones=${adaRead.phones.length} addresses=${adaRead.addresses.length}`);

const alanRead = await contactsRepo.readSingle(new Where(new Operator.Equals("id", alan.id)));
if (alanRead == null) throw new Error("read");
console.log(`read ${alanRead.id} ${alanRead.givenName} emails=${alanRead.emails.length} phones=${alanRead.phones.length} addresses=${alanRead.addresses.length}`);

const adaSaved = await contactsRepo.update({
  id: ada.id,
  givenName: "Ada Byron",
  genderTypeName: "Female",
  emails: [
    { id: ada.emails[0].id, emailTypeName: "Personal", address: ada.emails[0].address, displayName: "Ada", isPrimary: true } as Email,
    { emailTypeName: "Work", address: "ada@byron.example", displayName: "Ada work", isPrimary: false } as Email,
  ],
  phones: [
    { id: ada.phones[0].id, phoneTypeName: "Mobile", number: "555-0111", displayName: "Ada mobile", extention: "", isPrimary: true } as Phone,
  ],
  addresses: [
    { id: ada.addresses[0].id, addressTypeName: "Home", line1: ada.addresses[0].line1, city: "Cambridge", countryCode: "GB" } as Address,
  ],
} as Contact);
const adaAfter = await contactsRepo.readSingle(new Where(new Operator.Equals("id", ada.id)));
if (adaAfter == null) throw new Error("update");
console.log(
  `updated ${adaAfter.id} ${adaAfter.givenName} rows=${adaSaved.rowsAffected} emails=${[...adaAfter.emails].map((email) => email.address).sort().join(",")} phones=${[...adaAfter.phones].map((phone) => phone.number).sort().join(",")} ${adaAfter.addresses[0].city}`,
);

const alanSaved = await contactsRepo.update({
  id: alan.id,
  givenName: "Alan",
  genderTypeName: "Male",
  emails: [
    { id: alan.emails[0].id, emailTypeName: "Work", address: "alan@bletchley.example", displayName: "Alan", isPrimary: true } as Email,
  ],
  phones: [
    { id: alan.phones[0].id, phoneTypeName: "Work", number: "555-0201", displayName: "Alan office", extention: "", isPrimary: true } as Phone,
  ],
  addresses: [
    { id: alan.addresses[0].id, addressTypeName: "Work", line1: "2 Hut 8", city: alan.addresses[0].city, countryCode: "GB" } as Address,
  ],
} as Contact);
const alanAfter = await contactsRepo.readSingle(new Where(new Operator.Equals("id", alan.id)));
if (alanAfter == null) throw new Error("update");
console.log(
  `updated ${alanAfter.id} ${alanAfter.givenName} rows=${alanSaved.rowsAffected} emails=${alanAfter.emails.map((email) => email.address).join(",")} phones=${alanAfter.phones.map((phone) => phone.number).join(",")} ${alanAfter.addresses[0].line1}`,
);

const adaRemoved = await contactsRepo.delete(adaAfter);
if ((await contactsRepo.readSingle(new Where(new Operator.Equals("id", ada.id)))) != null) throw new Error("delete");
console.log(`deleted ${ada.id} rows=${adaRemoved.rowsAffected}`);

const alanRemoved = await contactsRepo.delete(alanAfter);
if ((await contactsRepo.readSingle(new Where(new Operator.Equals("id", alan.id)))) != null) throw new Error("delete");
console.log(`deleted ${alan.id} rows=${alanRemoved.rowsAffected}`);
