import { Join, StatementBuilder, type IQuery } from "../../index.ts";

export const contactsJoinQuery = (): IQuery =>
  StatementBuilder.createQuery(
    "contacts_base",
    [
      "contacts_base.id AS id",
      "contacts_base.first_name AS first_name",
      "contacts_base.last_name AS last_name",
      "contacts_base.email AS email",
      "contact_source.name AS contact_source_name",
      "contact_type.name AS contact_type_name",
      "address.id AS address_id",
      "address.line1 AS address_line1",
      "address.city AS address_city",
      "phone.id AS phone_id",
      "phone.number AS phone_number",
      "phone.label AS phone_label",
    ],
    undefined,
    [
      Join.inner("contact_source", "contacts_base.contact_source_id", "contact_source.id"),
      Join.inner("contact_type", "contacts_base.contact_type_id", "contact_type.id"),
      Join.left("address", "contacts_base.id", "address.contact_id"),
      Join.left("phone", "contacts_base.id", "phone.contact_id"),
    ],
  );
