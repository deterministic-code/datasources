import { expect } from "vitest";

type Nested = { length: number };
type View = { id: number; addresses: Nested[]; phones: Nested[] };

export const contactWith = <T extends View>(
  contacts: T[],
  addresses: number,
  phones: number,
): T => {
  const contact = contacts.find(
    (item) => item.addresses.length === addresses && item.phones.length === phones,
  );
  if (!contact) throw new Error(`no contact with ${addresses} addresses and ${phones} phones`);
  return contact;
};

export const expectContactsView = (contacts: View[], count: number) => {
  expect(contacts).toHaveLength(count);
  contactWith(contacts, 2, 1);
  contactWith(contacts, 0, 1);
  contactWith(contacts, 2, 0);
  contactWith(contacts, 0, 0);
};
