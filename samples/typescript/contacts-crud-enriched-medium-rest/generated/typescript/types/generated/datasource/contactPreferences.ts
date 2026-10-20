// schema-version: 1.0
/** Type ContactPreferences. */
export interface ContactPreferences {
  id: number;
  uuid: string;
  created: Date;
  updated: Date;
  version: string;
  contactId: number;
  key: string;
  locale: string;
  timezone: string;
}
