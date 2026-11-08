// schema-version: 1.0
import type { ContactsBase } from "../datasource/contactsBase";
import type { Address } from "./address";
import type { LocalePreference } from "./localePreference";
import type { Phone } from "./phone";
/** View Contact. */
type Dictionary<K extends PropertyKey, V> = Record<K, V>;
export const contact = (init: Partial<Contact>): Contact => ({ ...init } as Contact);
export interface Contact extends Omit<ContactsBase, "contactSourceId"> {
  contactSourceName: string;
  description: string | null;
  addresses: Address[];
  phones: Phone[];
  settings: Dictionary<string, string>;
  prefs: Dictionary<string, LocalePreference>;
}
