import type { TypeMap } from "../../mappers/TypeMap.ts";
import type { Contact } from "./Contact.ts";

export const contactsView: TypeMap<Contact> = {
  key: "id",
  fields: {
    id: "Id",
    first_name: "FirstName",
    last_name: "LastName",
    email: "Email",
    contact_source_name: "Name",
    contact_type_name: "ContactTypeName",
    addresses: {
      key: "AddressId",
      fields: {
        id: "AddressId",
        line1: "Line1",
        city: "City",
        address_type_name: "AddressTypeName",
      },
    },
    phones: {
      key: "PhoneId",
      fields: {
        id: "PhoneId",
        number: "Number",
        phone_type_name: "PhoneTypeName",
      },
    },
  },
};
