type AddressView = { id: number; line1: string; city: string };
type PhoneView = { id: number; number: string; label: string | null };

export type ContactsView = {
  id: number;
  first_name: string;
  last_name: string;
  email: string | null;
  contact_source_name: string;
  contact_type_name: string;
  addresses: AddressView[];
  phones: PhoneView[];
};

const byId = (left: { id: number }, right: { id: number }) => left.id - right.id;

export const asContactsView = (contacts: ContactsView[]): ContactsView[] =>
  [...contacts]
    .sort(byId)
    .map((contact) => ({
      id: contact.id,
      first_name: contact.first_name,
      last_name: contact.last_name,
      email: contact.email,
      contact_source_name: contact.contact_source_name,
      contact_type_name: contact.contact_type_name,
      addresses: [...contact.addresses]
        .map(({ id, line1, city }) => ({ id, line1, city }))
        .sort(byId),
      phones: [...contact.phones]
        .map(({ id, number, label }) => ({ id, number, label }))
        .sort(byId),
    }));
