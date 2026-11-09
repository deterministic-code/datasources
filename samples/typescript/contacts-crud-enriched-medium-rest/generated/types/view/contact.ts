// schema-version: 1.0
import type { Address } from "./address";
import type { LocalePreference } from "./localePreference";
import type { Phone } from "./phone";
/** View Contact. */
type Dictionary<K extends PropertyKey, V> = Record<K, V>;
export const contact = (init: Partial<Contact>): Contact => Object.assign(new Contact(), init);
export class Contact {
  id!: number;
  uuid: string = crypto.randomUUID();
  created: Date = new Date();
  updated: Date = new Date();
  version: string = Buffer.from([0]).toString("base64");
  firstName: string = "";
  lastName: string = "";
  email!: string | null;
  notes!: string | null;
  contactSourceName: string = "";
  description!: string | null;
  addresses: Address[] = [];
  phones: Phone[] = [];
  settings: Dictionary<string, string> = {};
  prefs: Dictionary<string, LocalePreference> = {};
}
