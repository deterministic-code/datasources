// schema-version: 1.0
import type { ContactsBase } from "../datasource/contactsBase";
import type { Address } from "./address";
import type { LocalePreferences } from "./localePreferences";
import type { Phone } from "./phone";
/** View Contact. */
type Dictionary<K extends PropertyKey, V> = Record<K, V>;
export interface Contact extends Omit<ContactsBase, "contactSourceId"> {
  contactSourceName: string;
  description: string | null;
  addresses: Address[];
  phones: Phone[];
  settings: Dictionary<string, string>;
  prefs: Dictionary<string, LocalePreferences>;
}
