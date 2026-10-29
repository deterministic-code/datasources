import { address, contact, email, phone } from "../generated/typescript/repositories/repositoryRegistry.ts";
import { Datasource, Operator, Where } from "@deterministic-code/datasources";

await using ds = Datasource.default;
const contactsRepo = ds.contacts;
const ada = await contactsRepo.create(contact({
  givenName: "Ada",
  genderTypeName: "Female",
  emails: [email({ emailTypeName: "Personal", address: "ada@example.com", displayName: "Ada", isPrimary: true })],
  phones: [phone({ phoneTypeName: "Mobile", number: "555-0100", displayName: "Ada mobile", extention: "", isPrimary: true })],
  addresses: [address({ addressTypeName: "Home", line1: "12 Analytical Engine", city: "London", countryCode: "GB" })],
}));
console.log(`created ${ada.id} ${ada.givenName} ${ada.emails[0].address} ${ada.phones[0].number} ${ada.addresses[0].line1}`);

const alan = await contactsRepo.create(contact({
  givenName: "Alan",
  genderTypeName: "Male",
  emails: [email({ emailTypeName: "Work", address: "alan@work.example", displayName: "Alan", isPrimary: true })],
  phones: [phone({ phoneTypeName: "Work", number: "555-0200", displayName: "Alan office", extention: "", isPrimary: true })],
  addresses: [address({ addressTypeName: "Work", line1: "1 Bletchley Park", city: "Bletchley", countryCode: "GB" })],
}));
console.log(`created ${alan.id} ${alan.givenName} ${alan.emails[0].address} ${alan.phones[0].number} ${alan.addresses[0].line1}`);

const adaRead = await contactsRepo.readSingle(new Where(new Operator.Equals("id", ada.id)));
if (adaRead == null) throw new Error("read");
console.log(`read ${adaRead.id} ${adaRead.givenName} emails=${adaRead.emails.length} phones=${adaRead.phones.length} addresses=${adaRead.addresses.length}`);

const alanRead = await contactsRepo.readSingle(new Where(new Operator.Equals("id", alan.id)));
if (alanRead == null) throw new Error("read");
console.log(`read ${alanRead.id} ${alanRead.givenName} emails=${alanRead.emails.length} phones=${alanRead.phones.length} addresses=${alanRead.addresses.length}`);

const adaSaved = await contactsRepo.update(contact({
  id: ada.id,
  givenName: "Ada Byron",
  genderTypeName: "Female",
  emails: [
    email({ id: ada.emails[0].id, emailTypeName: "Personal", address: ada.emails[0].address, displayName: "Ada", isPrimary: true }),
    email({ emailTypeName: "Work", address: "ada@byron.example", displayName: "Ada work", isPrimary: false }),
  ],
  phones: [
    phone({ id: ada.phones[0].id, phoneTypeName: "Mobile", number: "555-0111", displayName: "Ada mobile", extention: "", isPrimary: true }),
  ],
  addresses: [
    address({ id: ada.addresses[0].id, addressTypeName: "Home", line1: ada.addresses[0].line1, city: "Cambridge", countryCode: "GB" }),
  ],
}));
const adaAfter = await contactsRepo.readSingle(new Where(new Operator.Equals("id", ada.id)));
if (adaAfter == null) throw new Error("update");
console.log(
  `updated ${adaAfter.id} ${adaAfter.givenName} rows=${adaSaved.rowsAffected} emails=${[...adaAfter.emails].map((row) => row.address).sort().join(",")} phones=${[...adaAfter.phones].map((row) => row.number).sort().join(",")} ${adaAfter.addresses[0].city}`,
);

const alanSaved = await contactsRepo.update(contact({
  id: alan.id,
  givenName: "Alan",
  genderTypeName: "Male",
  emails: [
    email({ id: alan.emails[0].id, emailTypeName: "Work", address: "alan@bletchley.example", displayName: "Alan", isPrimary: true }),
  ],
  phones: [
    phone({ id: alan.phones[0].id, phoneTypeName: "Work", number: "555-0201", displayName: "Alan office", extention: "", isPrimary: true }),
  ],
  addresses: [
    address({ id: alan.addresses[0].id, addressTypeName: "Work", line1: "2 Hut 8", city: alan.addresses[0].city, countryCode: "GB" }),
  ],
}));
const alanAfter = await contactsRepo.readSingle(new Where(new Operator.Equals("id", alan.id)));
if (alanAfter == null) throw new Error("update");
console.log(
  `updated ${alanAfter.id} ${alanAfter.givenName} rows=${alanSaved.rowsAffected} emails=${alanAfter.emails.map((row) => row.address).join(",")} phones=${alanAfter.phones.map((row) => row.number).join(",")} ${alanAfter.addresses[0].line1}`,
);

const adaRemoved = await contactsRepo.delete(adaAfter);
if ((await contactsRepo.readSingle(new Where(new Operator.Equals("id", ada.id)))) != null) throw new Error("delete");
console.log(`deleted ${ada.id} rows=${adaRemoved.rowsAffected}`);

const alanRemoved = await contactsRepo.delete(alanAfter);
if ((await contactsRepo.readSingle(new Where(new Operator.Equals("id", alan.id)))) != null) throw new Error("delete");
console.log(`deleted ${alan.id} rows=${alanRemoved.rowsAffected}`);
