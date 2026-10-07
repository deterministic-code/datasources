// schema-version: 1.0
import type { ContactsBase } from "../datasource/contactsBase";
import type { Address } from "./address";
import type { Phone } from "./phone";
/** View Contact. */
export interface Contact extends Omit<ContactsBase, "contactSourceId" | "contactSourceUuid" | "contactSourceCreated" | "contactSourceUpdated" | "contactSourceVersion" | "contactSourceDescription"> {
  contactSourceName: string;
  addresses: Address[];
  phones: Phone[];
}
