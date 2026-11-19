// schema-version: 1.0
/** View ContactSettingsMap. */
export const contactSettingsMap = (init: Partial<ContactSettingsMap>): ContactSettingsMap => Object.assign(new ContactSettingsMap(), init);
export class ContactSettingsMap {
  key: string = "";
  value: string = "";
}
