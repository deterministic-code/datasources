import { WhereBuilder } from "../../builders/WhereBuilder.ts";
import { collectAsync } from "../../collectAsync.ts";
import { Address } from "./Address.ts";
import { Contact } from "./Contact.ts";
import type { ContactsService } from "./ContactsService.ts";
import { Phone } from "./Phone.ts";

const stamp = "2024-01-01T00:00:00Z";
const version = Buffer.alloc(0);

const base = <T extends object>(type: { new (): T }, extra: Partial<T>): T =>
  Object.assign(new type(), {
    uuid: "00000000-0000-4000-8000-000000000251",
    created: stamp,
    updated: stamp,
    version,
    ...extra,
  });

export const extraContact = (): Contact =>
  base(Contact, {
    id: 251,
    contact_source_id: 1,
    contact_type_id: 1,
    first_name: "Extra",
    last_name: "Contact",
    addresses: [base(Address, { id: 10001, line1: "9 Extra", city: "Oslo" })],
    phones: [base(Phone, { id: 10001, number: "555-extra" })],
  });

export const addExtraContact = async (service: ContactsService): Promise<Contact> => {
  const contact = extraContact();
  await service.add(contact);
  const loaded = (
    await collectAsync(service.get(WhereBuilder.equals("id", contact.id)))
  )[0];
  if (!loaded) throw new Error(`extra contact ${contact.id} missing`);
  return loaded;
};
