import type { Contact } from "./Contact.ts";

type AddressView = { id: number; line1: string; city: string; address_type_name: string };
type PhoneView = { id: number; number: string; phone_type_name: string };

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

export const fromContacts = (contacts: Contact[]): ContactsView[] =>
  contacts
    .map((contact) => ({
      id: contact.Id,
      first_name: contact.FirstName,
      last_name: contact.LastName,
      email: contact.Email,
      contact_source_name: contact.ContactSource.Name,
      contact_type_name: contact.ContactType.Name,
      addresses: contact.Addresses.map((address) => ({
        id: address.Id,
        line1: address.Line1,
        city: address.City,
        address_type_name: address.AddressType.Name,
      })).sort(byId),
      phones: contact.Phones.map((phone) => ({
        id: phone.Id,
        number: phone.Number,
        phone_type_name: phone.PhoneType.Name,
      })).sort(byId),
    }))
    .sort(byId);

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
        .map(({ id, line1, city, address_type_name }) => ({
          id,
          line1,
          city,
          address_type_name,
        }))
        .sort(byId),
      phones: [...contact.phones]
        .map(({ id, number, phone_type_name }) => ({ id, number, phone_type_name }))
        .sort(byId),
    }));
