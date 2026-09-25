import { Operator, Where } from "../../statements/index.ts";
import { collectAsync } from "../../collectAsync.ts";
import type { ICRUDRepository } from "../../ICRUDRepository.ts";
import { Address } from "./Address.ts";
import { Contact } from "./Contact.ts";
import type { ContactLookups } from "./contactLookups.ts";
import { Phone } from "./Phone.ts";

const stamp = "2024-01-01T00:00:00Z";
const version = Buffer.alloc(0);

const base = <T extends object>(type: { new (): T }, extra: Partial<T>): T =>
  Object.assign(new type(), {
    uuid: crypto.randomUUID(),
    created: stamp,
    updated: stamp,
    version,
    ...extra,
  });

export const extraContact = (ids: ContactLookups): Contact =>
  base(Contact, {
    contact_source_id: ids.contact_source_id,
    contact_type_id: ids.contact_type_id,
    first_name: "Extra",
    last_name: "Contact",
    addresses: [
      base(Address, { address_type_id: ids.address_type_id, line1: "9 Extra", city: "Oslo" }),
    ],
    phones: [base(Phone, { phone_type_id: ids.phone_type_id, number: "555-extra" })],
  });

export const addExtraContact = async (
  service: ICRUDRepository<Contact>,
  ids: ContactLookups,
): Promise<Contact> => {
  const contact = extraContact(ids);
  await service.create(contact);
  const loaded = (
    await collectAsync(service.read(new Where(new Operator.Equals("id", contact.id))))
  )[0];
  if (!loaded) throw new Error("extra contact missing after create");
  return loaded;
};
