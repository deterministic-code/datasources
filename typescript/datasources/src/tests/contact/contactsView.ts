import type { TypeMap } from "../../mappers/TypeMap.ts";
import type { Contact } from "./Contact.ts";

export const contactsView: TypeMap<Contact> = {
  key: "id",
  fields: {
    id: "id",
    first_name: "first_name",
    last_name: "last_name",
    email: "email",
    contact_source_name: "contact_source_name",
    contact_type_name: "contact_type_name",
    addresses: {
      key: "address_id",
      fields: {
        id: "address_id",
        line1: "address_line1",
        city: "address_city",
        address_type_name: "address_type_name",
      },
    },
    phones: {
      key: "phone_id",
      fields: {
        id: "phone_id",
        number: "phone_number",
        phone_type_name: "phone_type_name",
      },
    },
  },
};
