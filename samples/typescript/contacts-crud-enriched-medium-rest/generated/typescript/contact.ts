// schema-version: 1.0
import type { ContactsBase } from "../datasource/contactsBase";
import type { Address } from "./address";
import type { Phone } from "./phone";
/** View Contact. */
export interface Contact extends Omit<ContactsBase, "contact_source_id" | "contact_source_uuid" | "contact_source_created" | "contact_source_updated" | "contact_source_version" | "contact_source_description"> {
  contact_source_name: string;
  addresses: Address[];
  phones: Phone[];
}
