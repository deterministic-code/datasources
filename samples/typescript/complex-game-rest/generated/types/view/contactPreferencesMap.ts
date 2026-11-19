// schema-version: 1.0
import type { LocalePreference } from "./localePreference";
/** View ContactPreferencesMap. */
export const contactPreferencesMap = (init: Partial<ContactPreferencesMap>): ContactPreferencesMap => Object.assign(new ContactPreferencesMap(), init);
export class ContactPreferencesMap {
  key: string = "";
  value!: LocalePreference;
}
