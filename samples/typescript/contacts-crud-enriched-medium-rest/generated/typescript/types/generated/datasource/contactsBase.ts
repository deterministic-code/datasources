// schema-version: 1.0
/** Type ContactsBase. */
export interface ContactsBase {
  id: number;
  uuid: string;
  created: Date;
  updated: Date;
  version: string;
  contactSourceId: number;
  firstName: string;
  lastName: string;
  email: string | null;
  notes: string | null;
}
