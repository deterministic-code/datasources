import { Operator, Where } from "../../statements/index.ts";
import { collectAsync } from "../../collectAsync.ts";
import type { ICRUDRepository } from "../../ICRUDRepository.ts";
import { Address } from "./Address.ts";
import { AddressType } from "./AddressType.ts";
import { Contact } from "./Contact.ts";
import { ContactSource } from "./ContactSource.ts";
import { ContactType } from "./ContactType.ts";
import type { ContactLookups } from "./contactLookups.ts";
import { Phone } from "./Phone.ts";
import { PhoneType } from "./PhoneType.ts";

const stamp = new Date("2024-01-01T00:00:00Z");
const version = new Uint8Array();

const base = <T extends { Uuid: string; Created: Date; Updated: Date; Version: Uint8Array }>(
  type: { new (): T },
  extra: Partial<T>,
): T =>
  Object.assign(new type(), {
    Uuid: crypto.randomUUID(),
    Created: stamp,
    Updated: stamp,
    Version: version,
    ...extra,
  });

export const extraContact = (ids: ContactLookups): Contact =>
  base(Contact, {
    ContactSource: Object.assign(new ContactSource(), { Id: ids.ContactSourceId }),
    ContactType: Object.assign(new ContactType(), { Id: ids.ContactTypeId }),
    FirstName: "Extra",
    LastName: "Contact",
    Addresses: [
      base(Address, {
        AddressType: Object.assign(new AddressType(), { Id: ids.AddressTypeId }),
        Line1: "9 Extra",
        City: "Oslo",
      }),
    ],
    Phones: [
      base(Phone, {
        PhoneType: Object.assign(new PhoneType(), { Id: ids.PhoneTypeId }),
        Number: "555-extra",
      }),
    ],
  });

export const addExtraContact = async (
  service: ICRUDRepository<Contact>,
  ids: ContactLookups,
): Promise<Contact> => {
  const contact = extraContact(ids);
  const created = await service.create(contact);
  if (!(created.Id > 0)) throw new Error("create returned no id");
  const loaded = (
    await collectAsync(service.read(new Where(new Operator.Equals("Id", contact.Id))))
  )[0];
  if (!loaded) throw new Error("extra contact missing after create");
  return loaded;
};
